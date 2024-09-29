use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::{
    lambda::{user_books_table_fields as fields, USER_BOOKS_SHARE_INDEX_NAME, USER_BOOKS_TABLE_NAME},
    Book,
};

use tracing::info;

/// Returns a list of URLs for all user photos for the given ISBN.
/// It is possible that the data retrieval fails, but there is nothing the caller can do
/// to fix that, so the function returns an empty list.
pub(crate) async fn get_photo_share_urls(isbn: u64, share_id: &str) -> Vec<String> {
    let client = Client::new(&aws_config::load_from_env().await);

    let user_ids = match client
        .query()
        .table_name(USER_BOOKS_TABLE_NAME)
        .index_name(USER_BOOKS_SHARE_INDEX_NAME)
        .key_condition_expression("#isbn = :isbn AND #share = :share")
        .expression_attribute_names("#isbn", fields::ISBN)
        .expression_attribute_values(":isbn", AttributeValue::N(isbn.to_string()))
        .expression_attribute_names("#share", fields::SHARE_ID)
        .expression_attribute_values(":share", AttributeValue::S(share_id.to_string()))
        .send()
        .await
    {
        Ok(v) => match v.items {
            // convert the items into books
            Some(items) => {
                // loop thru the records
                items
                    .iter()
                    .filter_map(|item| item.get(fields::UID).map(|v| attr_s_to_string(v.clone())))
                    .collect::<Vec<_>>()
            }

            None => {
                info!("No books found for share_id {}, isbn: {}", share_id, isbn);
                Vec::new()
            }
        },
        Err(e) => {
            info!(
                "Failed to get items for share_id: {}, isbn: {}, error: {:?}",
                share_id, isbn, e
            );
            return Vec::new();
        }
    };

    // check if there is more than one record, which is an error
    let user_id = match user_ids.len() {
        0 => {
            info!("No records found for share_id: {}, isbn: {}", share_id, isbn);
            return Vec::new();
        }
        1 => user_ids[0].clone(),
        _ => {
            info!("More than one record found for share_id: {}, isbn: {}", share_id, isbn);
            return Vec::new();
        }
    };

    // get the list of photos for the user book
    let photos = match client
        .query()
        .table_name(USER_BOOKS_TABLE_NAME)
        .key_condition_expression("#user_id = :user_id AND #isbn = :isbn")
        .expression_attribute_names("#user_id", fields::UID)
        .expression_attribute_values(":user_id", AttributeValue::S(user_id.clone()))
        .expression_attribute_names("#isbn", fields::ISBN)
        .expression_attribute_values(":isbn", AttributeValue::N(isbn.to_string()))
        .send()
        .await
    {
        Ok(v) => match v.items {
            // convert the items into books
            Some(items) => {
                // loop thru the records
                items
                    .iter()
                    .filter_map(|item| match item.get(fields::PHOTO_IDS) {
                        Some(AttributeValue::Ss(v)) => Some(v.to_owned()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            }

            None => {
                info!("No books found for user_id {}, isbn: {}", user_id, isbn);
                Vec::new()
            }
        },
        Err(e) => {
            info!("Failed to get books for {}: {:?}", user_id, e);
            return Vec::new();
        }
    };

    // flatten the list of photos
    // there should be only one record for the user and the ISBN
    let photos = photos.into_iter().flatten().collect::<Vec<_>>();

    // build the full URL out of the photo IDs
    let photos = Book::hydrate_photos(&user_id, isbn, photos);

    info!(
        "Returning {} photo URLs for share_id: {}, user_id: {}, isbn: {}",
        photos.len(),
        share_id,
        user_id,
        isbn
    );
    photos
}

/// Converts the AttributeValue into a string
/// Returns an empty string if the value is not a string
fn attr_s_to_string(v: AttributeValue) -> String {
    match v {
        AttributeValue::S(v) => v,
        _ => "".to_string(),
    }
}
