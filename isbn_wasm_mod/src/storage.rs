use crate::google::Volumes;
use crate::utils::log;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use web_sys::Window;

#[derive(Deserialize, Serialize, Debug)]
pub struct BookRecord {
    #[serde(default)]
    pub isbn: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Books {
    books: Vec<BookRecord>,
}

impl BookRecord {
    /// Adds a not to an existing book record, creates a new record if the ISBN is not found.
    /// The book record is stored in the local storage (front-end only access).
    /// Fails silently if the record cannot be stored.
    /// TODO: Add error handling.
    pub(crate) fn store_locally(self, runtime: &Window) {
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

        let mut book_record = match ls.get_item(&self.isbn) {
            Ok(Some(v)) => match serde_json::from_str::<BookRecord>(&v) {
                Ok(v) => {
                    log!("Book record found in local storage");
                    v
                }
                Err(e) => {
                    log!("Failed to parse local storage book record: {:?}", e);
                    return;
                }
            },
            Ok(None) => {
                log!("Book record not found in local storage");
                self
            }
            Err(e) => {
                log!("Failed to get book record from local storage: {:?}", e);
                return;
            }
        };

        // add the note to the book record
        book_record.timestamp = Utc::now();

        // replace the record in the database
        let isbn = book_record.isbn.clone();
        let book_record = match serde_json::to_string(&book_record) {
            Ok(v) => v,
            Err(e) => {
                log!("Failed to serialize book record for {isbn}: {:?}", e);
                return;
            }
        };
        match ls.set_item(&isbn, &book_record) {
            Ok(()) => log!("Book record saved"),
            Err(e) => log!("Failed to save book record: {:?}", e),
        }
    }

    /// Converts the very first Volume into Self, if it exists.
    /// Otherwise, returns None.
    pub(crate) fn from_google_books(volumes: Volumes, isbn: &str) -> Option<Self> {
        if volumes.items.is_empty() {
            return None;
        }

        Some(Self {
            isbn: isbn.to_string(),
            title: volumes.items[0].volume_info.title.clone(),
            author: volumes.items[0].volume_info.authors[0].clone(),
            timestamp: Utc::now(),
        })
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
            let value = match ls.get_item(&key) {
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

            // parse the string value into a book record
            let book_record = match serde_json::from_str::<BookRecord>(&value) {
                Ok(v) => v,
                Err(e) => {
                    log!("Failed to parse local storage book record for {key}: {:?}", e);
                    continue;
                }
            };

            books.push(book_record);
        }

        // the items in the local storage are never sorted
        // sort the list to make the latest scanned book come first
        books.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(Books { books })
    }
}
