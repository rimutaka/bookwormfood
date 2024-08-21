use crate::google::get_book_data;
use crate::http_req::{execute_http_request, IdToken};
use crate::utils::log;
use anyhow::{bail, Error, Result};
use bookwormfood_types::{Book, ReadStatus};
use chrono::Utc;
use web_sys::Window;

/// Adds a not to an existing book record, creates a new record if the ISBN is not found.
/// The book record is stored in the local storage (front-end only access).
/// Fails silently if the record cannot be stored.
/// TODO: Add error handling.
pub(crate) async fn save(book: &Book, runtime: &Window, id_token: &Option<IdToken>) {
    // get the reference to the local storage
    let ls = match runtime.local_storage() {
        Ok(Some(v)) => v,
        Err(e) => {
            log!("Failed to get local storage: {:?}", e);
            return;
        }
        _ => {
            log!("Local storage not available (OK(None))");
            return;
        }
    };

    // replace the record in the database
    let key = book.isbn.clone();
    let value = match serde_json::to_string(book) {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to serialize book record for {key}: {:?}", e);
            return;
        }
    };
    match ls.set_item(&key, &value) {
        Ok(()) => log!("Book {key} saved in local storage"),
        Err(e) => log!("Failed to save book {key} record: {:?}", e),
    }

    // save the book to the cloud DB
    // TODO: do something with the result
    if id_token.is_some() {
        _ = send_to_ddb(book, runtime, id_token).await;
    } else {
        log!("No token to send the book to the cloud DB");
    }
}

/// Updates the status of a book record in the local storage.
/// Returns the updated book details back.
/// Returns an error if the book cannot be found in LS or in GoogleBooks.
pub(crate) async fn update_status(
    runtime: &Window,
    isbn: &str,
    status: Option<ReadStatus>,
    id_token: &Option<IdToken>,
) -> Result<Book> {
    // get the book data
    let book = match get(runtime, isbn, id_token).await? {
        Some(mut v) => {
            // exit if the previous status is the same as the new one
            // but I can't see how that may even happen if the UI behaves
            if status == v.read_status {
                log!("New status == old for {isbn}");
                return Ok(v);
            };

            // update the status
            v.timestamp_update = Utc::now();
            v.read_status = status;
            v
        }
        None => {
            bail!("Book not found for ISBN {isbn}");
        }
    };

    // connect to the local storage
    let ls = match runtime.local_storage() {
        Ok(Some(v)) => v,
        Err(e) => {
            bail!("Failed to get local storage: {:?}", e);
        }
        _ => {
            bail!("Local storage not available (OK(None))");
        }
    };

    // save the book record
    match serde_json::to_string(&book) {
        Ok(v) => match ls.set_item(isbn, &v) {
            Ok(()) => log!("Book record updated"),
            Err(e) => bail!("Failed to save book record: {:?}", e),
        },
        Err(e) => {
            bail!("Failed to serialize book record for {isbn}: {:?}", e);
        }
    };

    // save the book to the cloud DB
    // TODO: do something with the result
    if id_token.is_some() {
        _ = send_to_ddb(&book, runtime, id_token).await;
    } else {
        log!("No token to send the book to the cloud DB");
    }

    Ok(book)
}

/// Fetches a book record from the local storage by ISBN.
/// if the book is not found in the local storage it fetches the book data from Google Books.
/// - Error - something went wrong
/// - None - the book was not found
pub(crate) async fn get(runtime: &Window, isbn: &str, id_token: &Option<IdToken>) -> Result<Option<Book>> {
    // try to get the book from the local storage first

    // connect to the local storage
    let ls = match runtime.local_storage() {
        Ok(Some(v)) => v,
        Err(e) => {
            bail!("Failed to get local storage: {:?}", e);
        }
        _ => {
            bail!("Local storage not available (OK(None))");
        }
    };

    // return book details from LS by isbn, if found
    if let Ok(Some(v)) = ls.get_item(isbn) {
        log!("Found in local storage: {isbn}");
        match serde_json::from_str::<Book>(&v) {
            Ok(v) => return Ok(Some(v)),
            Err(e) => {
                log!("Failed to parse local storage book record for {isbn}: {:?}", e);
            }
        };
    };

    // if the book is not found in the local storage, fetch it from Google Books
    let book = match get_book_data(isbn, runtime).await {
        Ok(mut v) => match v.items.pop() {
            Some(v) => Book {
                isbn: isbn.to_string(),
                timestamp_update: Utc::now(),
                cover: v.volume_info.get_thumbnail(None),
                title: Some(v.volume_info.title.clone()),
                authors: Some(v.volume_info.authors.clone()),
                volume_info: Some(v.volume_info),
                read_status: None,
                timestamp_sync: None,
            },
            None => {
                bail!("Nothing in Google Books for ISBN {isbn}");
            }
        },

        Err(e) => {
            log!("Failed to get book data from Google Books for {isbn}: {:?}", e);
            bail!("Cannot get book data from Google Books for ISBN {isbn}");
        }
    };

    // store the book record in the local storage
    save(&book, runtime, id_token).await;

    Ok(Some(book))
}

/// Deletes the book from the local storage.
/// Does nothing if the book is not found in the local storage.
pub(crate) async fn delete(runtime: &Window, isbn: &str) -> Result<()> {
    // connect to the local storage
    let ls = match runtime.local_storage() {
        Ok(Some(v)) => v,
        Err(e) => {
            bail!("Failed to get local storage: {:?}", e);
        }
        _ => {
            bail!("Local storage not available (OK(None))");
        }
    };

    // delete the book from LS by isbn
    match ls.remove_item(isbn) {
        Ok(()) => log!("Book {isbn} removed from local storage"),
        Err(e) => {
            log!("Failed to remove local storage book record for {isbn}: {:?}", e);
            bail!("Failed to remove local storage book record for {isbn}");
        }
    };

    Ok(())
}

/// Sends the book data to the cloud DB via a lambda.
/// The lambda decides what to store and where.
/// Logs any errors.
async fn send_to_ddb(book: &Book, runtime: &Window, id_token: &Option<IdToken>) -> Result<Book> {
    log!("Sending book data to lambda: {}", book.isbn);

    let url = "https://bookwormfood.com/sync.html";

    // TODO: error handling is a mess
    execute_http_request::<Book, Book>(url, Some(book), runtime, id_token)
        .await
        .map_err(|_| Error::msg("Failed to send book data to lambda"))
}
