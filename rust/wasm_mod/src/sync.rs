use crate::http_req::{execute_http_request, HttpMethod};
use crate::utils::{get_local_storage, log};
use anyhow::{bail, Error, Result};
use bookwormfood_types::{Book, Books, IdToken, ISBN_URL_PARAM_NAME, SYNC_HTML_URL};
use std::collections::HashSet;
use web_sys::Window;

/// Try to save the book to the cloud DB and update the sync status in the local storage.
/// No action is taken if there is no token or the book is already sync'd.
/// The sync is only from local to cloud.
/// All errors are logged.
pub(crate) async fn sync_book(isbn: u64, runtime: &Window, id_token: &Option<IdToken>) -> Result<()> {
    // nothing to do if the user is not logged in
    if id_token.is_none() {
        log!("No token. Sync skipped.");
        return Ok(());
    }

    let ls = get_local_storage(runtime)?;

    let local_book = match ls.get_item(&isbn.to_string()) {
        Ok(Some(v)) => {
            log!("Found in local storage: {isbn}");
            match serde_json::from_str::<Book>(&v) {
                Ok(v) => v,
                Err(e) => {
                    log!("Failed to parse local storage book record for {isbn}: {:?}", e);
                    bail!("Failed to parse local storage book record");
                }
            }
        }
        Ok(None) => {
            log!("Book not found in local storage: {isbn}");
            bail!("Book not found in local storage");
        }
        Err(e) => {
            log!("Failed to get local storage book record for {isbn}: {:?}", e);
            bail!("Failed to get local storage book record");
        }
    };

    // check if the books needs a sync
    if let Some(v) = local_book.timestamp_sync {
        if v > local_book.timestamp_update {
            log!("Book sync is current: {isbn}");
            return Ok(());
        }
    }

    log!("Sending book data to lambda: {}", local_book.isbn);

    // some fields are never saved in the cloud
    let mut cloud_book = Book::new(local_book.isbn);
    // these fields are saved in the cloud
    cloud_book.authors = local_book.authors.clone();
    cloud_book.read_status = local_book.read_status;
    cloud_book.timestamp_update = local_book.timestamp_update;
    cloud_book.title = local_book.title.clone();

    // send the data to the cloud DB
    // set the new sync timestamp to the current time on success or None on failure
    let book = if execute_http_request::<Book, ()>(SYNC_HTML_URL, HttpMethod::Post(cloud_book), runtime, id_token)
        .await
        .is_ok()
    {
        log!("Book sync'd with the cloud DB");
        local_book.with_new_sync_timestamp()
    } else {
        log!("Failed to sync the book with the cloud DB");
        local_book.without_sync_timestamp()
    };

    // try to save the book with the updated sync field in the local storage
    match serde_json::to_string(&book) {
        Ok(v) => match ls.set_item(&book.isbn.to_string(), &v) {
            Ok(()) => log!("Sync status updated to {:?}", book.timestamp_sync),
            Err(e) => {
                log!("Failed to update sync status: {:?}", e);
            }
        },
        Err(e) => {
            log!("Failed to serialize book record for {}: {:?}", book.isbn, e);
        }
    };

    Ok(())
}

/// Get the list of books from the cloud DB and update the local storage.
/// Returns:
/// - the updated list of books on success
/// - None if there was no change
/// - Error with a user-friendly message on error
///
/// TODO: send unsync'd books to the cloud
pub(crate) async fn sync_books(books: Books, runtime: &Window, id_token: &Option<IdToken>) -> Result<Option<Books>> {
    // nothing to do if the user is not logged in
    if id_token.is_none() {
        log!("No token. Sync skipped.");
        return Ok(None);
    }

    // get the list of books from the lambda
    let cloud_books = match execute_http_request::<(), Books>(SYNC_HTML_URL, HttpMethod::Get, runtime, id_token).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            log!("No books in the cloud DB");
            return Ok(None);
        }
        Err(e) => {
            log!("Failed to get books from the cloud DB: {:?}", e);
            return Err(Error::msg("Failed to get books from the cloud DB"));
        }
    };

    log!("Cloud books: {}, local: {}", cloud_books.books.len(), books.books.len());

    // index the local books by ISBN for faster lookups
    let mut local_books = books
        .books
        .into_iter()
        .map(|v| (v.isbn, v))
        .collect::<std::collections::HashMap<_, _>>();

    // find local books that need to be updated from the cloud
    let books_to_update = cloud_books
        .books
        .iter()
        .filter_map(|cloud_book| {
            match local_books.get_mut(&cloud_book.isbn) {
                Some(local_book) => {
                    // the book is already in the local storage
                    // check if the cloud book is newer
                    if cloud_book.timestamp_update > local_book.timestamp_update {
                        // the cloud book is newer
                        // update the local book
                        log!("Merge from cloud for ISBN: {}", cloud_book.isbn);
                        local_book.merge_from(cloud_book);
                        Some(local_book.isbn)
                    } else {
                        // the local book is newer
                        None
                    }
                }
                None => None,
            }
        })
        .collect::<HashSet<_>>();

    // find books that don't exist locally and need to be added from the cloud
    let books_to_add = cloud_books
        .books
        .into_iter()
        .filter_map(|cloud_book| {
            match local_books.get(&cloud_book.isbn) {
                Some(_) => None,
                None => {
                    // the book is not in the local storage
                    // add it
                    log!("Cloud book not in LS: {}", cloud_book.isbn);
                    Some(cloud_book)
                }
            }
        })
        .collect::<Vec<_>>();

    // exit now if there is nothing to add or update
    if books_to_add.is_empty() && books_to_update.is_empty() {
        log!("No new books to add or update");
        return Ok(None);
    };

    log!(
        "Cloud books to add: {}, update: {}",
        books_to_add.len(),
        books_to_update.len()
    );

    let ls = get_local_storage(runtime)?;

    // convert the hashmap back to a Vec list of books ans save any updated books along the way
    let mut books = Books {
        books: local_books
            .into_values()
            .map(|book| {
                if books_to_update.contains(&book.isbn) {
                    let book = book.with_new_sync_timestamp();
                    match serde_json::to_string(&book) {
                        Ok(v) => match ls.set_item(&book.isbn.to_string(), &v) {
                            Ok(()) => {
                                log!("Updated in local storage: {}", book.isbn);
                                book
                            }
                            Err(e) => {
                                log!("Failed to update sync status for {}: {:?}", book.isbn, e);
                                // this makes no sense because the record in LS may have a different value
                                book.without_sync_timestamp()
                            }
                        },
                        Err(e) => {
                            log!("Failed to serialize book record for {}: {:?}", book.isbn, e);
                            book.without_sync_timestamp()
                        }
                    }
                } else {
                    book
                }
            })
            .collect(),
    };

    // reallocate the local list of books to accommodate the new books
    books.books.reserve(books_to_add.len());

    // save the new books to the local storage and add them to the list of local books
    for cloud_book in books_to_add {
        // try to save the book with the updated sync field in the local storage
        let cloud_book = cloud_book.with_new_sync_timestamp();
        let cloud_book = match serde_json::to_string(&cloud_book) {
            Ok(v) => match ls.set_item(&cloud_book.isbn.to_string(), &v) {
                Ok(()) => {
                    log!("Added to local storage: {}", cloud_book.isbn);
                    cloud_book
                }
                Err(e) => {
                    log!("Failed to update sync status for {}: {:?}", cloud_book.isbn, e);
                    // this makes no sense because the record in LS may have a different value
                    cloud_book.without_sync_timestamp()
                }
            },
            Err(e) => {
                log!("Failed to serialize book record for {}: {:?}", cloud_book.isbn, e);
                cloud_book.without_sync_timestamp()
            }
        };

        books.books.push(cloud_book);
    }

    books.sort();

    Ok(Some(books))
}

/// Try to delete the book from the cloud DB.
/// By this time the book should not exist in the local storage.
/// No action is taken if the book fails to delete - it will reappear on the next sync.
pub(crate) async fn delete_book(isbn: &str, runtime: &Window, id_token: &Option<IdToken>) -> Result<()> {
    // nothing to do if the user is not logged in
    if id_token.is_none() {
        log!("No token. Sync skipped.");
        return Ok(());
    }

    log!("Sending book deletion request to lambda: {}", isbn);

    // the lambda only needs the ISBN for this operation
    let url = [SYNC_HTML_URL, "?", ISBN_URL_PARAM_NAME, "=", isbn].concat();

    // send the ISBN to the cloud DB
    if execute_http_request::<Book, ()>(&url, HttpMethod::Delete, runtime, id_token)
        .await
        .is_ok()
    {
        log!("Book deleted from the cloud DB");
        Ok(())
    } else {
        log!("Failed to delete the book from the cloud DB");
        bail!("Failed to delete the book from the cloud DB");
    }
}
