use crate::USER_BOOKS_TABLE_NAME;
use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::Book;
use tracing::info;

pub(crate) async fn save(book: &Book, client: &Client, uid: &str) -> Result<(), Error> {
    match client
        .put_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .item("uid", AttributeValue::S(uid.to_string()))
        .item("isbn", AttributeValue::N(book.isbn.clone()))
        .item("title", attr_val_s(&book.title))
        .item("authors", attr_val_ss(&book.authors))
        .item("updated", AttributeValue::S(book.timestamp_update.to_rfc3339()))
        .item(
            "read_status",
            book.read_status
                .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::S(v.to_string())),
        )
        .send()
        .await
    {
        Ok(_) => {
            info!("Book saved in DDB");
            Ok(())
        }
        Err(e) => {
            info!("Failed to save book {}/{}: {:?}", uid, book.isbn, e);
            Err(Error::msg("Failed to save book".to_string()))
        }
    }
}

fn attr_val_s(v: &Option<String>) -> AttributeValue {
    v.as_ref()
        .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::S(v.clone()))
}

fn attr_val_ss(v: &Option<Vec<String>>) -> AttributeValue {
    v.as_ref()
        .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::Ss(v.clone()))
}
