use crate::google::get_book_data;
use crate::utils::{get_local_storage, log};
use anyhow::{bail, Result};
use bookwormfood_types::{Book, ReadStatus};
use chrono::Utc;
use web_sys::Window;

/// Adds a not to an existing book record, creates a new record if the ISBN is not found.
/// The book record is stored in the local storage (front-end only access).
/// Fails silently if the record cannot be stored.
pub(crate) async fn save(book: &Book, runtime: &Window) -> Result<()> {
    // get the reference to the local storage
    let ls = get_local_storage(runtime)?;

    // replace the record in the database
    let key = book.isbn;
    let value = match serde_json::to_string(&book) {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to serialize book record for {key}: {:?}", e);
            bail!("Book {key} not saved locally");
        }
    };

    // log!("Book to save: {value}");

    match ls.set_item(&key.to_string(), &value) {
        Ok(()) => {
            log!("Book {key} saved in local storage");
        }
        Err(e) => {
            log!("Failed to save book {key} record: {:?}", e);
            bail!("Book {key} not saved locally");
        }
    }

    Ok(())
}

/// Adds Google Books data to the book record.
/// Returns an unchanged book if the call fails or no data was found.
/// All errors are logged.
pub(crate) async fn enhance_from_google_books(book: Book, runtime: &Window) -> Book {
    if book.volume_info.is_some() {
        log!("Insufficient Google Books data: {}", book.isbn);
        return book;
    }

    log!("Insufficient details: {}", book.isbn);
    match get_book_data(book.isbn, runtime).await {
        Ok(mut v) => match v.items.pop() {
            Some(v) => {
                let mut book = book;
                book.cover = v.volume_info.get_thumbnail(None);
                book.title = Some(v.volume_info.title.clone());
                book.authors = Some(v.volume_info.authors.clone());
                book.volume_info = Some(v.volume_info);

                book
            }
            None => {
                log!("Nothing in Google Books for ISBN {}", book.isbn);
                book
            }
        },

        Err(e) => {
            log!("Failed to get book data from Google Books for {}: {:?}", book.isbn, e);
            book
        }
    }
}

/// Updates the status of a book record in the local storage.
/// Returns the updated book details back.
/// Returns an error if the book cannot be found in LS or in GoogleBooks.
pub(crate) async fn update_status(runtime: &Window, isbn: u64, status: Option<ReadStatus>) -> Result<Book> {
    // get the book data
    let book = match get(runtime, isbn).await? {
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
        Ok(v) => match ls.set_item(&isbn.to_string(), &v) {
            Ok(()) => log!("Book record updated"),
            Err(e) => bail!("Failed to save book record: {:?}", e),
        },
        Err(e) => {
            bail!("Failed to serialize book record for {isbn}: {:?}", e);
        }
    };

    Ok(book)
}

/// Fetches a book record from the local storage by ISBN.
/// if the book is not found in the local storage it fetches the book data from Google Books.
/// - Error - something went wrong
/// - None - the book was not found
pub(crate) async fn get(runtime: &Window, isbn: u64) -> Result<Option<Book>> {
    // try to get the book from the local storage first

    // connect to the local storage
    let ls = get_local_storage(runtime)?;

    // get book details from LS by isbn or create a shell for populating it with data from other sources
    let local_book = match ls.get_item(&isbn.to_string()) {
        Ok(Some(v)) => {
            log!("Found in local storage: {isbn}");
            // log!("{}",v);

            match serde_json::from_str::<Book>(&v) {
                Ok(v) => v,
                Err(e) => {
                    log!("Failed to parse local storage book record for {isbn}: {:?}", e);
                    Book::new(isbn)
                }
            }
        }
        _ => Book::new(isbn),
    };

    // log!("{:?}", local_book);

    // check if the book has everything the user needs
    // log!(
    //     "Needs enhancing: {}, {}, {}",
    //     local_book.title.is_none(),
    //     local_book.authors.is_none(),
    //     local_book.volume_info.is_none()
    // );

    if !local_book.needs_enhancing() {
        return Ok(Some(local_book));
    }

    // if the book is not found in the local storage, fetch it from Google Books
    let book = enhance_from_google_books(local_book, runtime).await;

    // store the book record in the local storage and sync with the cloud DB
    // TODO: add error handling
    let _ = save(&book, runtime).await;

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
