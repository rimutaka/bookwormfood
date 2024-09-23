use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::lambda::{Uid, USER_BOOKS_TABLE_NAME, user_books_table_fields as fields};
use chrono::Utc;
use tracing::info;

/// Adds a photo ID to a user book record.
pub(crate) async fn add_photo(uid: Uid, isbn: String, photo_id: String) -> Result<(), crate::Error> {
    let client = Client::new(&aws_config::load_from_env().await);

    match client
        .update_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .update_expression("ADD photo_ids :photo_ids SET updated = :updated")
        .key(fields::UID, AttributeValue::S(uid.0.clone()))
        .key(fields::ISBN, AttributeValue::N(isbn.clone()))
        .expression_attribute_values(
            [":", fields::PHOTO_IDS].concat(),
            AttributeValue::Ss([photo_id.clone()].to_vec()),
        )
        .expression_attribute_values(
            [":", fields::UPDATED].concat(),
            AttributeValue::S(Utc::now().to_rfc3339()),
        )
        .send()
        .await
    {
        Ok(_) => {
            info!("Photo saved in DDB");
            Ok(())
        }
        Err(e) => {
            info!("Failed to save photo {} / {} / {} : {:?}", uid.0, isbn, photo_id, e);
            Err(anyhow::Error::msg("Failed to save photo".to_string()).into())
        }
    }
}
