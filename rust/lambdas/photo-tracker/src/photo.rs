use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::lambda::{user_books_table_fields as fields, Uid, USER_BOOKS_TABLE_NAME};
use chrono::Utc;
use tracing::info;

/// Adds a photo ID to a user book record.
pub(crate) async fn add_photo_to_ddb(uid: Uid, isbn: String, photo_id: String) -> Result<(), crate::Error> {
    match update_ddb(
        uid.clone(),
        isbn.clone(),
        photo_id.clone(),
        "ADD photo_ids :photo_ids SET updated = :updated",
    )
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

/// Deletes a photo ID from a user book record.
pub(crate) async fn remove_photo_from_ddb(uid: Uid, isbn: String, photo_id: String) -> Result<(), crate::Error> {
    match update_ddb(
        uid.clone(),
        isbn.clone(),
        photo_id.clone(),
        "DELETE photo_ids :photo_ids SET updated = :updated",
    )
    .await
    {
        Ok(_) => {
            info!("Photo deleted from DDB");
            Ok(())
        }
        Err(e) => {
            info!("Failed to delete photo {} / {} / {} : {:?}", uid.0, isbn, photo_id, e);
            Err(anyhow::Error::msg("Failed to delete photo".to_string()).into())
        }
    }
}

/// A reusable part of calling DDB for adding or removing a photo ID.
async fn update_ddb(
    uid: Uid,
    isbn: String,
    photo_id: String,
    update_expression: &str,
) -> Result<
    aws_sdk_dynamodb::operation::update_item::UpdateItemOutput,
    aws_smithy_runtime_api::client::result::SdkError<
        aws_sdk_dynamodb::operation::update_item::UpdateItemError,
        aws_smithy_runtime_api::http::Response,
    >,
> {
    let client = Client::new(&aws_config::load_from_env().await);

    client
        .update_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .update_expression(update_expression)
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
}
