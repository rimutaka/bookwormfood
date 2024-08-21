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
use crate::http_req::execute_http_request;
use crate::utils::log;
use web_sys::Window;
use bookwormfood_types::google::Volumes;

/// Fetches book data from Google Books API
pub(crate) async fn get_book_data(isbn: &str, runtime: &Window) -> super::Result<Volumes> {
    log!("Querying google books for: {isbn}");

    let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");

    execute_http_request::<Volumes, u8>(&url, None, runtime, &None).await
}
