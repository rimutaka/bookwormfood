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
    #[serde(default)]
    pub isbn: String,
    #[serde(default)]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub status: Option<BookStatus>,
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
#[derive(Copy, Clone, Deserialize, Serialize, Debug)]
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
    /// Does nothing if the record doesn't exist.
    pub(crate) fn update_status(runtime: &Window, isbn: &str, status: Option<BookStatus>) -> Result<Self> {
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

        // get the book's record
        let book = match ls.get_item(isbn) {
            Ok(Some(v)) => v,
            Err(e) => {
                bail!("Failed to get book from local storage: {:?}", e);
            }
            Ok(None) => {
                bail!("Book not found in local storage");
            }
        };

        // parse and update the book record
        let book = match serde_json::from_str::<Book>(&book) {
            Ok(mut v) => {
                v.timestamp = Utc::now();
                v.status = status;
                v
            }
            Err(e) => {
                bail!("Failed to parse book record: {:?}", e);
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

            books.push(book);
        }

        // the items in the local storage are never sorted
        // sort the list to make the latest scanned book come first
        books.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(Books { books })
    }
}
