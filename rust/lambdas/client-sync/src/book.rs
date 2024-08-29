use crate::{Email, Uid, USER_BOOKS_TABLE_NAME};
use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::{Book, Books, ReadStatus};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use tracing::info;

const FIELD_UID: &str = "uid"; // used in a query
const FIELD_EMAIL: &str = "email"; // from JWT, never returned to the caller
const FIELD_ISBN: &str = "isbn";
const FIELD_TITLE: &str = "title";
const FIELD_AUTHORS: &str = "authors";
const FIELD_UPDATED: &str = "updated";
const FIELD_READ_STATUS: &str = "read_status";

/// Save a book in the user_books table.
/// Replaces existing records unconditionally.
pub(crate) async fn save(book: &Book, client: &Client, uid: Uid, email: Email) -> Result<(), Error> {
    match client
        .put_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .item(FIELD_UID, AttributeValue::S(uid.0.clone()))
        .item(FIELD_EMAIL, AttributeValue::S(email.0))
        .item(FIELD_ISBN, AttributeValue::N(book.isbn.to_string()))
        .item(FIELD_TITLE, attr_val_s(&book.title))
        .item(FIELD_AUTHORS, attr_val_ss(&book.authors))
        .item(FIELD_UPDATED, AttributeValue::S(book.timestamp_update.to_rfc3339()))
        .item(
            FIELD_READ_STATUS,
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
            info!("Failed to save book {}/{}: {:?}", uid.0, book.isbn, e);
            Err(Error::msg("Failed to save book".to_string()))
        }
    }
}

/// Returns all book records for the given user.
/// Returns an empty list if no records found.
pub(crate) async fn get_by_user(client: &Client, uid: Uid) -> Result<Books, Error> {
    let books = match client
        .query()
        .table_name(USER_BOOKS_TABLE_NAME)
        .key_condition_expression("#uid = :uid")
        .expression_attribute_names("#uid", FIELD_UID)
        .expression_attribute_values(":uid", AttributeValue::S(uid.0.clone()))
        .send()
        .await
    {
        Ok(v) => match v.items {
            // convert the items into books
            Some(items) => {
                let mut books = Vec::with_capacity(items.len());
                // loop thru the records
                'item: for item in items {
                    let mut book = Book::new(Book::FAKE_ISBN);

                    // iterate through the list of attributes for the record
                    // instead of looking them up by name
                    for attr in item {
                        match attr.0.as_str() {
                            FIELD_ISBN => {
                                book.isbn = match attr_to_string(attr.1).parse::<u64>() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        info!("Invalid ISBN for user {}", uid.0);
                                        continue 'item;
                                    }
                                }
                            }
                            FIELD_TITLE => book.title = attr_to_option(attr.1),
                            FIELD_AUTHORS => {
                                book.authors = match attr.1 {
                                    AttributeValue::Ss(v) => Some(v),
                                    _ => None,
                                }
                            }
                            FIELD_UPDATED => {
                                book.timestamp_update = match DateTime::parse_from_rfc3339(&attr_to_string(attr.1)) {
                                    Ok(v) => v.into(),
                                    Err(_) => DateTime::<Utc>::MIN_UTC,
                                }
                            }
                            FIELD_READ_STATUS => book.read_status = ReadStatus::from_str(&attr_to_string(attr.1)).ok(),
                            _ => {}
                        }
                    }

                    // there is potential for an incomplete record if ISBN/Updated fields are missing

                    books.push(book);
                }

                Books { books }
            }
            None => {
                info!("No books found for user {}", uid.0);
                Books { books: Vec::new() }
            }
        },
        Err(e) => {
            info!("Failed to get books for {}: {:?}", uid.0, e);
            return Err(Error::msg("Failed to save book".to_string()));
        }
    };

    info!("Returning {} books for {}", books.books.len(), uid.0);
    Ok(books)
}

/// Deletes a book from user_books table.
pub(crate) async fn delete(isbn: &str, client: &Client, uid: Uid) -> Result<(), Error> {
    match client
        .delete_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .key(FIELD_UID, AttributeValue::S(uid.0.clone()))
        .key(FIELD_ISBN, AttributeValue::S(isbn.to_string()))
        .send()
        .await
    {
        Ok(_) => {
            info!("Book deleted from DDB: {}/{}", uid.0, isbn);
            Ok(())
        }
        Err(e) => {
            info!("Failed to delete book {}/{}: {:?}", uid.0, isbn, e);
            Err(Error::msg("Failed to delete book".to_string()))
        }
    }
}

///Converts the value into an AttributeValue
fn attr_val_s(v: &Option<String>) -> AttributeValue {
    v.as_ref()
        .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::S(v.clone()))
}

///Converts the value into an AttributeValue
fn attr_val_ss(v: &Option<Vec<String>>) -> AttributeValue {
    v.as_ref()
        .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::Ss(v.clone()))
}

/// Converts the AttributeValue into a string
fn attr_to_string(v: AttributeValue) -> String {
    match v {
        AttributeValue::S(v) => v,
        _ => "".to_string(),
    }
}

/// Converts the AttributeValue into an option-string
fn attr_to_option(v: AttributeValue) -> Option<String> {
    match v {
        AttributeValue::S(v) => Some(v),
        _ => None,
    }
}
