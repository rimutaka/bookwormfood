use crate::google::Volumes;
use crate::utils::log;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use web_sys::Window;

#[derive(Deserialize, Serialize, Debug)]
pub struct BookNote {
    pub timestamp: DateTime<Utc>,
    pub note: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookRecord {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub notes: Vec<BookNote>,
}

#[derive(Deserialize, Serialize, Debug)]
struct BookDB {
    books: Vec<BookRecord>,
}

impl BookRecord {
    /// Adds a not to an existing book record, creates a new record if the ISBN is not found.
    /// The book record is stored in the local storage (front-end only access).
    /// Fails silently if the record cannot be stored.
    /// TODO: Add error handling.
    pub(crate) fn add_note(self, runtime: &Window, note: String) {
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
                Ok(v) => v,
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
        book_record.notes.push(BookNote {
            timestamp: Utc::now(),
            note,
        });

        // replace the record in the database
        let isbn = book_record.isbn.clone();
        let book_record = match serde_json::to_string(&book_record) {
            Ok(v) => v,
            Err(e) => {
                log!("Failed to serialize book record: {:?}", e);
                return;
            }
        };
        match ls.set_item(&isbn, &book_record) {
            Ok(()) => log!("Book record updated"),
            Err(e) => log!("Failed to update book record: {:?}", e),
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
            notes: Vec::new(),
        })
    }
}
