use storage::BookRecord;
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

    // GoogleBooks seems to have the most accurate and most up to data on all books
    // get it first, send back to the UI and use the bits of info from there to do more
    // fetching from other sources
    let resp = match google::get_book_data(&isbn, &runtime).await {
        Ok(v) => {
            log!("Book data received");
            WasmResponse::GoogleBooks(Some(WasmResult::Ok(v)))
        }

        Err(e) => {
            log!("Failed to get book data");
            log!("{:?}", e);
            WasmResponse::GoogleBooks(Some(WasmResult::Err(format!("{:?}", e))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    report_progress(resp.to_string());

    // store the book record in the local storage, if possible
    if let WasmResponse::GoogleBooks(Some(Ok(v))) = resp {
        log!("Storing book in local storage");
        if let Some(v) = BookRecord::from_google_books(v, &isbn) {
            v.store_locally(&runtime);
        }
    }
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
            log!("Book list retrieved");
            WasmResponse::LocalBooks(Some(WasmResult::Ok(v)))
        }
        Err(e) => {
            log!("Failed to get list of books");
            log!("{:?}", e);
            WasmResponse::LocalBooks(Some(WasmResult::Err(format!("{:?}", e))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    // send the response back to the UI thread
    report_progress(resp.to_string());
}
