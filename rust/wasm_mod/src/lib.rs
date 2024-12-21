use bookworm_types::{jwt, Books, IdToken, ReadStatus};
pub use http_req::AUTH_HEADER;
use sync::{sync_book, sync_books};
use utils::get_runtime;
use wasm_bindgen::prelude::*;
use wasm_response::{report_progress, WasmResponse, WasmResult};
use web_sys::FileList;

#[macro_use]
pub(crate) mod utils;
mod book;
mod books;
pub mod google;
mod http_req;
mod photos;
mod sync;
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
pub async fn get_book_data(isbn: String, id_token: Option<IdToken>, share_id: Option<String>) {
    log!("Getting book data for ISBN: {isbn}, share ID: {:?}", share_id);

    let isbn = match isbn.parse::<u64>() {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to parse ISBN {isbn}. It's a bug. {:?}", e);
            return;
        }
    };

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
    let resp = match book::get(&runtime, isbn).await {
        Ok(Some(v)) => {
            // log!("{:?}", v);
            // hydrate the book for the front-end
            let v = if let Some(user) = jwt::get_user_details(&id_token) {
                v.hydrate(&user.id)
            } else {
                v
            };
            log!("Sending book data to UI");
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Ok(v))))
        }
        Ok(None) => {
            log!("Sending a blank msg to UI");
            WasmResponse::LocalBook(Box::new(None))
        }

        Err(e) => {
            log!("Sending an error msg to UI");
            // log!("{:?}", e);
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Err(format!("{:?}", e)))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    report_progress(resp.to_string());

    // get additional photos for the book if there is a share ID
    if let Some(share_id) = share_id {
        log!("Inside share ID block");
        if let WasmResponse::LocalBook(book) = resp {
            log!("Inside WASM resp block");
            // get the book data
            let book = *book;
            if let Some(Ok(mut book)) = book {
                log!("Inside book block");
                // get the list of user photos for this book
                let photo_urls = photos::get_shared_photo_urls(&runtime, &share_id, isbn).await;

                // add the photo URLs to the book data
                if !photo_urls.is_empty() {
                    match book.photos {
                        Some(mut photos) => {
                            photos.extend(photo_urls);
                            // duplicates are possible if the current user is looking at her own share
                            photos.dedup();
                            book.photos = Some(photos);
                            log!("Photos added to the book (extend): {:?}", book.photos);
                        }
                        None => {
                            book.photos = Some(photo_urls);
                            log!("Photos added to the book (set): {:?}", book.photos);

                        }
                    }

                    // send the response back to the UI thread
                    let resp = WasmResponse::LocalBook(Box::new(Some(WasmResult::Ok(book))));
                    report_progress(resp.to_string());
                }
            }
        }
    }

    let _ = sync_book(isbn, &runtime, &id_token).await;
}

/// Returns the list of previously scanned books from the local storage.
/// See `fn report_progress()` for more details.
#[wasm_bindgen]
pub async fn get_scanned_books(id_token: Option<IdToken>, with_cloud_sync: bool) {
    log!(
        "Getting the list of books from local storage. Sync: {}",
        with_cloud_sync
    );

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // if this happened it would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // get the list of books from the local storage
    let local_books = books::get(&runtime);

    // get Books from local storage and wrap them into a response struct
    let resp = match &local_books {
        Ok(v) => {
            log!("Book list retrieved: {}", v.books.len());
            // TODO: get rid of this clone
            WasmResponse::LocalBooks(Box::new(Some(WasmResult::Ok(v.lean_copy()))))
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

    if !with_cloud_sync {
        log!("Skipping cloud sync");
        return;
    }

    // proceed to getting potentially more user books from the cloud DB
    let local_books = match local_books {
        Ok(v) => v,
        Err(_) => Books { books: Vec::new() },
    };

    // try to sync the books with the cloud DB
    let resp = match sync_books(local_books, &runtime, &id_token).await {
        Ok(Some(v)) => {
            log!("Update book list from cloud DB: {}", v.books.len());
            WasmResponse::LocalBooks(Box::new(Some(WasmResult::Ok(v))))
        }
        Ok(None) => {
            // no need to send anything to the UI
            log!("No new books from the cloud DB");
            return;
        }
        Err(_) => {
            log!("Getting list of cloud DB books failed");
            // TODO: send a report back to the UI
            return;
        }
    };

    // send the updated list of books to the UI
    report_progress(resp.to_string());
}

/// Updates the status of a book in the local storage.
/// Returns `WasmResponse::LocalBook::Ok` in a message if successful.
#[wasm_bindgen]
pub async fn update_book_status(isbn: String, status: Option<ReadStatus>, id_token: Option<IdToken>) {
    log!("Updating book status in local storage");

    let isbn = match isbn.parse::<u64>() {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to parse ISBN {isbn}. It's a bug. {:?}", e);
            return;
        }
    };

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
    let resp = match book::update_status(&runtime, isbn, status).await {
        Ok(v) => {
            log!("Book status updated");
            // hydrate the book for the front-end
            let v = if let Some(user) = jwt::get_user_details(&id_token) {
                v.hydrate(&user.id)
            } else {
                v
            };
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

    let _ = sync_book(isbn, &runtime, &id_token).await;
}

/// Deletes a book from the local storage.
/// Returns error or success via an async message.
#[wasm_bindgen]
pub async fn delete_book(isbn: String, id_token: Option<IdToken>) {
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
    let resp = match book::delete(&runtime, &isbn).await {
        Ok(_) => {
            log!("Book deleted");
            WasmResponse::Deleted(Box::new(Some(WasmResult::Ok(isbn.clone()))))
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

    // TODO: handle possible errors
    let _ = crate::sync::delete_book(&isbn, &runtime, &id_token).await;
}

/// Uploads a file to S3.
/// Returns error or success via an async message.
#[wasm_bindgen]
pub async fn upload_pic(isbn: String, files: FileList, id_token: Option<IdToken>) {
    log!("Uploading an image to S3");

    let isbn = match isbn.parse::<u64>() {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to parse ISBN {isbn}. It's a bug. {:?}", e);
            return;
        }
    };

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
    let resp = match photos::upload(&runtime, isbn, files, &id_token).await {
        Some(v) => {
            log!("Photos uploaded");
            // hydrate the book for the front-end
            let v = if let Some(user) = jwt::get_user_details(&id_token) {
                v.hydrate(&user.id)
            } else {
                v
            };
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Ok(v))))
        }
        None => {
            log!("Photo upload failed for {isbn}");
            WasmResponse::LocalBook(Box::new(Some(WasmResult::Err("Failed to upload photo".to_string()))))
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    // send the response back to the UI thread
    report_progress(resp.to_string());
}
