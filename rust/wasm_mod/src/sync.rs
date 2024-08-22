use crate::http_req::{execute_http_request, IdToken};
use crate::utils::{get_local_storage, log};
use anyhow::{bail, Result};
use bookwormfood_types::Book;
use chrono::Utc;
use web_sys::Window;

/// Try to save the book to the cloud DB and update the sync status in the local storage.
/// No action is taken if there is no token or the book is already sync'd.
/// The sync is only from local to cloud.
/// All errors are logged.
pub(crate) async fn sync_book(isbn: &str, runtime: &Window, id_token: &Option<IdToken>) -> Result<()> {
    // nothing to do if the user is not logged in
    if id_token.is_none() {
        log!("No token. Sync skipped.");
        return Ok(());
    }

    let ls = get_local_storage(runtime)?;

    let book = match ls.get_item(isbn) {
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
    if let Some(v) = book.timestamp_sync {
        if v > book.timestamp_update {
            log!("Book sync is current: {isbn}");
            return Ok(());
        }
    }

    log!("Sending book data to lambda: {}", book.isbn);

    // some fields are never saved in the cloud
    let book = Book {
        volume_info: None,
        cover: None,
        timestamp_sync: None,
        // these fields are saved in the cloud
        authors: book.authors.clone(),
        isbn: book.isbn.clone(),
        read_status: book.read_status,
        timestamp_update: book.timestamp_update,
        title: book.title.clone(),
    };

    let url = "https://bookwormfood.com/sync.html";

    let mut book = book;
    // send the data to the cloud DB
    // set the new sync timestamp to the current time on success or None on failure
    book.timestamp_sync = if execute_http_request::<Book, ()>(url, Some(&book), runtime, id_token)
        .await
        .is_ok()
    {
        log!("Book sync'd with the cloud DB");
        Some(Utc::now())
    } else {
        log!("Failed to sync the book with the cloud DB");
        None
    };

    // try to save the book with the updated sync field in the local storage
    match serde_json::to_string(&book) {
        Ok(v) => match ls.set_item(&book.isbn, &v) {
            Ok(()) => log!("Sync status updated to {:?}", book.timestamp_sync),
            Err(e) => {
                log!("Failed to update sync status: {:?}", e);
                // this makes no sense because the record in LS may have a different value
                book.timestamp_sync = None;
            }
        },
        Err(e) => {
            log!("Failed to serialize book record for {}: {:?}", book.isbn, e);
            book.timestamp_sync = None;
        }
    };

    Ok(())
}
