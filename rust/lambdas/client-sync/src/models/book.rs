use crate::USER_BOOKS_TABLE_NAME;
use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;
use wasm_mod::{models::book::Book, models::book_status::ReadStatus};

/// Simplified book details for saving in the database.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DdbBook {
    pub isbn: String,
    pub updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_status: Option<ReadStatus>,
    pub title: String,
    pub authors: Vec<String>,
}

/// Converts a Book to a DdbBook.
impl TryFrom<&Option<String>> for DdbBook {
    type Error = anyhow::Error;

    fn try_from(payload: &Option<String>) -> Result<Self, Self::Error> {
        // try to deser the body into a book
        let book = match payload {
            Some(v) => match serde_json::from_str::<Book>(v) {
                Ok(v) => v,
                Err(e) => {
                    info!("Failed to parse payload: {:?}", e);
                    return Err(Error::msg("Failed to parse payload into Book"));
                }
            },
            None => {
                info!("Empty input");
                return Err(Error::msg("Empty input"));
            }
        };

        // validate isbn
        if ((book.isbn.len() == 13 && book.isbn.starts_with("97")) || book.isbn.len() == 10)
            && book.isbn.parse::<u64>().is_ok()
        {
            info!("ISBN: {}", book.isbn);
        } else if book.isbn.parse::<u64>().is_err() {
            info!("Invalid ISBN: {}", book.isbn);
            return Err(Error::msg("Invalid ISBN"));
        }

        info!("ISBN: {}", book.isbn);

        Ok(DdbBook {
            isbn: book.isbn,
            updated: book.timestamp_update,
            read_status: book.read_status,
            title: book.volume_info.title,
            authors: book.volume_info.authors,
        })
    }
}


impl DdbBook {
    /// Saves the book to the database. Returns an error if the save fails.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn save(&self, client: &Client, uid: &str) -> Result<(), Error> {
        match client
            .put_item()
            .table_name(USER_BOOKS_TABLE_NAME)
            .item("uid", AttributeValue::S(uid.to_string()))
            .item("isbn", AttributeValue::N(self.isbn.clone()))
            .item("title", AttributeValue::S(self.title.clone()))
            .item("authors", AttributeValue::Ss(self.authors.clone()))
            .item("updated", AttributeValue::S(self.updated.to_rfc3339()))
            .item(
                "read_status",
                self.read_status
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
                info!("Failed to save book {}/{}: {:?}", uid, self.isbn, e);
                Err(Error::msg("Failed to save book".to_string()))
            }
        }
    }
}
