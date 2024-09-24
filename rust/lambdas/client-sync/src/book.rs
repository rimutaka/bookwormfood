use crate::{Email, Uid, USER_BOOKS_TABLE_NAME};
use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bookwormfood_types::{lambda::user_books_table_fields as fields, Book, Books, ReadStatus};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use tracing::info;

/// Save a book in the user_books table.
/// Replaces existing records unconditionally.
pub(crate) async fn save(book: &Book, client: &Client, uid: Uid, email: Email) -> Result<(), Error> {
    // this has to be an update to prevent overwriting photo IDs
    const UPDATE_EXPRESSION: &str =
        "SET email = :email, title = :title, authors = :authors, read_status = :read_status, updated = :updated";

    match client
        .update_item()
        .table_name(USER_BOOKS_TABLE_NAME)
        .update_expression(UPDATE_EXPRESSION)
        .key(fields::UID, AttributeValue::S(uid.0.clone()))
        .key(fields::ISBN, AttributeValue::N(book.isbn.to_string()))
        .expression_attribute_values([":", fields::EMAIL].concat(), AttributeValue::S(email.0.clone()))
        .expression_attribute_values([":", fields::TITLE].concat(), attr_val_s(&book.title))
        .expression_attribute_values([":", fields::AUTHORS].concat(), attr_val_ss(&book.authors))
        .expression_attribute_values(
            [":", fields::READ_STATUS].concat(),
            book.read_status
                .map_or_else(|| AttributeValue::Null(true), |v| AttributeValue::S(v.to_string())),
        )
        .expression_attribute_values(
            [":", fields::UPDATED].concat(),
            AttributeValue::S(Utc::now().to_rfc3339()),
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
        .expression_attribute_names("#uid", fields::UID)
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
                            fields::ISBN => {
                                book.isbn = match attr_to_isbn(attr.1) {
                                    Some(v) => v,
                                    None => continue 'item,
                                }
                            }
                            fields::TITLE => book.title = attr_to_option(attr.1),
                            fields::AUTHORS => {
                                book.authors = match attr.1 {
                                    AttributeValue::Ss(v) => Some(v),
                                    _ => None,
                                }
                            }
                            fields::UPDATED => {
                                book.timestamp_update = match DateTime::parse_from_rfc3339(&attr_s_to_string(attr.1)) {
                                    Ok(v) => v.into(),
                                    Err(_) => DateTime::<Utc>::MIN_UTC,
                                }
                            }
                            fields::READ_STATUS => {
                                book.read_status = ReadStatus::from_str(&attr_s_to_string(attr.1)).ok()
                            }
                            fields::PHOTO_IDS => {
                                info!("Photo IDs: {:?}", attr.1);
                                book.photos = match attr.1 {
                                    AttributeValue::Ss(v) => Some(v),
                                    _ => None,
                                }
                            }
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
        .key(fields::UID, AttributeValue::S(uid.0.clone()))
        .key(fields::ISBN, AttributeValue::S(isbn.to_string()))
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
/// Returns an empty string if the value is not a string
fn attr_s_to_string(v: AttributeValue) -> String {
    match v {
        AttributeValue::S(v) => v,
        _ => "".to_string(),
    }
}

/// Converts a numeric filed AttributeValue into a string
/// Returns an empty string if the value is not a string
fn attr_to_isbn(v: AttributeValue) -> Option<u64> {
    match v {
        AttributeValue::N(v) => match v.parse::<u64>() {
            Ok(isbn) => Some(isbn),
            Err(e) => {
                info!("Invalid ISBN. Val: {}, err: {}", v, e);
                None
            }
        },
        _ => None,
    }
}

/// Converts the AttributeValue into an option-string
fn attr_to_option(v: AttributeValue) -> Option<String> {
    match v {
        AttributeValue::S(v) => Some(v),
        _ => None,
    }
}
