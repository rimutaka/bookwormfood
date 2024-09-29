use aws_sdk_dynamodb::{
    types::{AttributeValue, ReturnValue},
    Client,
};
use bookwormfood_types::lambda::{user_books_table_fields as fields, USER_BOOKS_TABLE_NAME};
use chrono::Utc;
use tracing::info;

/// Adds a photo ID to a user book record.
pub(crate) async fn add_photo_to_ddb(user_id: &str, isbn: String, photo_id: String) -> Result<(), crate::Error> {
    match update_ddb(
        user_id,
        isbn.clone(),
        photo_id.clone(),
        "ADD photo_ids :photo_ids SET updated = :updated", // TODO: replace attribute names with constants
    )
    .await
    {
        Ok(_) => {
            info!("Photo saved in DDB");
            Ok(())
        }
        Err(e) => {
            info!("Failed to save photo {} / {} / {} : {:?}", user_id, isbn, photo_id, e);
            Err(anyhow::Error::msg("Failed to save photo".to_string()).into())
        }
    }
}

/// Deletes a photo ID from a user book record.
pub(crate) async fn remove_photo_from_ddb(user_id: &str, isbn: String, photo_id: String) -> Result<(), crate::Error> {
    match update_ddb(
        user_id,
        isbn.clone(),
        photo_id.clone(),
        "DELETE photo_ids :photo_ids SET updated = :updated", // TODO: replace attribute names with constants
    )
    .await
    {
        Ok(_) => {
            info!("Photo deleted from DDB");
            Ok(())
        }
        Err(e) => {
            info!("Failed to delete photo {} / {} / {} : {:?}", user_id, isbn, photo_id, e);
            Err(anyhow::Error::msg("Failed to delete photo".to_string()).into())
        }
    }
}

/// A reusable part of calling DDB for adding or removing a photo ID.
async fn update_ddb(
    user_id: &str,
    isbn: String,
    photo_id: String,
    update_expression: &str,
) -> Result<(), crate::Error> {
    let client = Client::new(&aws_config::load_from_env().await);

    // update the list of photos and return the updated item
    let updated_item = match client
        .update_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .update_expression(update_expression)
        .key(fields::UID, AttributeValue::S(user_id.to_owned()))
        .key(fields::ISBN, AttributeValue::N(isbn.clone()))
        .expression_attribute_values(
            [":", fields::PHOTO_IDS].concat(),
            AttributeValue::Ss([photo_id.clone()].to_vec()),
        )
        .expression_attribute_values(
            [":", fields::UPDATED].concat(),
            AttributeValue::S(Utc::now().to_rfc3339()),
        )
        .return_values(ReturnValue::AllNew)
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return Err(e.into());
        }
    };

    // set the share attribute to the photo id if none was set before
    if let Some(att) = updated_item.attributes {
        // info!("Updated item: {:?}", att);
        if !att.contains_key(fields::SHARE_ID) {
            info!("Setting share to {photo_id}");
            let update_expression = ["SET #share = :", fields::SHARE_ID].concat();
            match client
                .update_item()
                .table_name(USER_BOOKS_TABLE_NAME)
                .update_expression(update_expression)
                .key(fields::UID, AttributeValue::S(user_id.to_owned()))
                .key(fields::ISBN, AttributeValue::N(isbn.clone()))
                .expression_attribute_names("#share", fields::SHARE_ID) // share is a reserved word
                .expression_attribute_values([":", fields::SHARE_ID].concat(), AttributeValue::S(photo_id.clone()))
                .send()
                .await
            {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e.into()),
            }
        }
    };

    // return OK if the share value was set earlier
    Ok(())
}
