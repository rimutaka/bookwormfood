use crate::google::{get_book_data, VolumeInfo};
use crate::utils::log;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Window;

/// An internal representation of a book record.
/// Stored in the local storage.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    /// This ISBN may differ from the key in the local storage or the industry IDs in the Google Books API.
    #[serde(default)]
    pub isbn: String,
    /// When the book was last updated.
    #[serde(default)]
    pub timestamp: DateTime<Utc>,
    /// Where the reader is with the book.
    #[serde(default)]
    pub status: Option<BookStatus>,
    /// The cover image URL from Google Books API.
    #[serde(default)]
    pub cover: Option<String>,
    /// The book details from Google Books API
    #[serde(default)]
    pub volume_info: VolumeInfo,
}

/// A list of book records.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Books {
    pub books: Vec<Book>,
}

/// Where the reader is with the book.
/// Defaults to None.
#[wasm_bindgen]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum BookStatus {
    ToRead,
    Read,
    Liked,
}

impl Book {
    /// Adds a not to an existing book record, creates a new record if the ISBN is not found.
    /// The book record is stored in the local storage (front-end only access).
    /// Fails silently if the record cannot be stored.
    /// TODO: Add error handling.
    pub(crate) fn store_locally(&self, runtime: &Window) {
        // get the book record from the database

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
        let key = self.isbn.clone();
        let value = match serde_json::to_string(self) {
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
    }

    /// Updates the status of a book record in the local storage.
    /// Returns the updated book details back.
    /// Returns an error if the book cannot be found in LS or in GoogleBooks.
    pub(crate) async fn update_status(runtime: &Window, isbn: &str, status: Option<BookStatus>) -> Result<Self> {
        // get the book data
        let book = match Book::get(runtime, isbn).await? {
            Some(mut v) => {
                // exit if the previous status is the same as the new one
                // but I can't see how that may even happen if the UI behaves
                if status == v.status {
                    log!("New status == old for {isbn}");
                    return Ok(v);
                };

                // update the status
                v.timestamp = Utc::now();
                v.status = status;
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

        Ok(book)
    }

    /// Fetches a book record from the local storage by ISBN.
    /// if the book is not found in the local storage it fetches the book data from Google Books.
    /// - Error - something went wrong
    /// - None - the book was not found
    pub(crate) async fn get(runtime: &Window, isbn: &str) -> Result<Option<Self>> {
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
                Some(v) => Self {
                    isbn: isbn.to_string(),
                    timestamp: Utc::now(),
                    status: None,
                    cover: v.volume_info.get_thumbnail(None),
                    volume_info: v.volume_info,
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
        book.store_locally(runtime);

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
}

impl Books {
    /// Returns a sorted array of all book records stored locally.
    /// Errors are logged.
    pub(crate) fn get(runtime: &Window) -> Result<Self> {
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

        // get the total number of records
        let number_of_records = match ls.length() {
            Ok(v) => v,
            Err(e) => {
                bail!("Failed to get local storage length: {:?}", e);
            }
        };

        // init the books array to the max possible size
        let mut books = Vec::with_capacity(number_of_records.try_into().unwrap_or_else(|_e| {
            log!("Failed to convert local storage length {number_of_records} to usize. It's a bug.");
            0
        }));

        // get one key at a time (inefficient, but the best we have with Local Storage)
        for i in 0..number_of_records {
            // get the key by index
            let key = match ls.key(i) {
                Ok(Some(v)) => v,
                Ok(None) => {
                    log!("Key {i} not found in local storage");
                    continue;
                }
                Err(e) => {
                    log!("Failed to get key {i} from local storage: {:?}", e);
                    continue;
                }
            };

            // get value by key
            let book = match ls.get_item(&key) {
                Ok(Some(v)) => v,
                Ok(None) => {
                    log!("Value not found in local storage: {key}");
                    continue;
                }
                Err(e) => {
                    log!("Failed to get value from local storage for {key}: {:?}", e);
                    continue;
                }
            };

            // log!("{book}");

            // parse the string value into a book record
            let book = match serde_json::from_str::<Book>(&book) {
                Ok(v) => v,
                Err(e) => {
                    log!("Failed to parse local storage book record for {key}: {:?}", e);
                    continue;
                }
            };

            // log!("{:?}", book);

            // ignore books with no titles because it is likely to be a corrupted record
            // from the format change or a bug
            // the user will have no benefit from such records
            // TODO: compare the ISBN inside Book and the key - they may differ
            // TODO: delete these ignored records
            if book.volume_info.title.is_empty() {
                log!("Empty title for {key}");
                continue;
            }

            books.push(book);
        }

        // the items in the local storage are randomly sorted
        // sort the list to make the latest scanned book come first
        books.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(Books { books })
    }
}
