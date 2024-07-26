use storage::Book;
use utils::get_runtime;
use wasm_bindgen::prelude::*;
use wasm_response::{report_progress, WasmResponse, WasmResult};

#[macro_use]
pub(crate) mod utils;
pub mod google;
mod http_req;
pub mod storage;
pub mod wasm_response;

/// All error handling in this crate is based on either retrying a request after some time
/// or exiting gracefully.
#[derive(Debug, Clone, PartialEq)]
pub enum RetryAfter {
    Seconds(i64),
    Never,
}

/// The result type that should be used in place of std::Result
/// throughout the app
pub type Result<T> = std::result::Result<T, RetryAfter>;

/// The main entry point for the UI thread to request book data.
/// Multiple responses are sent back via `progress.js` to the UI thread.
/// See `fn report_progress()` for more details.
#[wasm_bindgen]
pub async fn get_book_data(isbn: String) {
    log!("Getting book data for ISBN: {isbn}");

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // this would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // get the book details from either the local storage or the Google Books API
    let resp = match Book::get(&runtime, &isbn).await {
        Ok(Some(v)) => {
            log!("Sending book data to UI");
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Ok(v))))
        }
        Ok(None) => {
            log!("Sending a blank msg to UI");
            WasmResponse::LocalBook(Box::new(None))
        }

        Err(e) => {
            log!("Failed to get book data");
            log!("{:?}", e);
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Err(format!("{:?}", e)))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    report_progress(resp.to_string());
}

/// Returns the list of previously scanned books from the local storage.
/// See `fn report_progress()` for more details.
#[wasm_bindgen]
pub async fn get_scanned_books() {
    log!("Getting the list of books from local storage");

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // if this happened it would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // get Books from local storage and wrap them into a response struct
    let resp = match storage::Books::get(&runtime) {
        Ok(v) => {
            log!("Book list retrieved: {}", v.books.len());
            WasmResponse::LocalBooks(Box::new(Some(WasmResult::Ok(v))))
        }
        Err(e) => {
            log!("Failed to get list of books");
            log!("{:?}", e);
            WasmResponse::LocalBooks(Box::new(Some(WasmResult::Err(format!("{:?}", e)))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    // send the response back to the UI thread
    report_progress(resp.to_string());
}

/// Updates the status of a book in the local storage.
/// Returns `WasmResponse::LocalBook::Ok` in a message if successful.
#[wasm_bindgen]
pub async fn update_book_status(isbn: String, status: Option<storage::BookStatus>) {
    log!("Updating book status in local storage");

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // if this happened it would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // get Books from local storage and wrap them into a response struct
    let resp = match storage::Book::update_status(&runtime, &isbn, status).await {
        Ok(v) => {
            log!("Book status updated");
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Ok(v))))
        }
        Err(e) => {
            log!("Failed to update book status");
            log!("{:?}", e);
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Err(format!("{:?}", e)))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    // send the response back to the UI thread
    report_progress(resp.to_string());
}

/// Deletes a book from the local storage.
/// Returns error or success via an async message.
#[wasm_bindgen]
pub async fn delete_book(isbn: String) {
    log!("Deleting book from local storage");

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // if this happened it would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // get Books from local storage and wrap them into a response struct
    let resp = match storage::Book::delete(&runtime, &isbn).await {
        Ok(_) => {
            log!("Book deleted");
            WasmResponse::Deleted(Box::new(Some(WasmResult::Ok(isbn))))
        }
        Err(e) => {
            log!("Failed to delete book {isbn}");
            log!("{:?}", e);
            WasmResponse::Deleted(Box::new(Some(WasmResult::Err(format!("{:?}", e)))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    // send the response back to the UI thread
    report_progress(resp.to_string());
}
