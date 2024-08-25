/// Logic for fetching book data from Google Books API
///
/// Volume search by ISBN:
/// - https://www.googleapis.com/books/v1/volumes?q=isbn:9781761186769
///
/// Response: isbn_wasm_mod/data-samples/google-books-volume.json
///
/// API Reference: https://developers.google.com/books/docs/v1/reference/volumes#resource
///
//
use crate::http_req::{execute_http_request, HttpMethod};
use crate::utils::log;
use crate::{Result, RetryAfter};
use bookwormfood_types::google::Volumes;
use web_sys::Window;

/// Fetches book data from Google Books API
pub(crate) async fn get_book_data(isbn: &str, runtime: &Window) -> Result<Volumes> {
    log!("Querying google books for: {isbn}");

    let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");

    match execute_http_request::<u8, Volumes>(&url, HttpMethod::Get, runtime, &None).await {
        Ok(Some(v)) => Ok(v),
        Ok(None) => {
            log!("Blank response from Google for {isbn}");
            Err(RetryAfter::Never)
        }
        Err(e) => {
            log!("Failed to get book data for {isbn}");
            log!("{:?}", e);
            Err(e)
        }
    }
}
