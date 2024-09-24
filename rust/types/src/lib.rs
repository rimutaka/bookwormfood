use crate::google::VolumeInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub mod google;
#[cfg(not(target_arch = "wasm32"))]
pub mod lambda;

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

/// The name of the authorisation header containing the ID token with the user email.
pub const AUTH_HEADER: &str = "x-books-authorization";

/// Value: `isbn`. The URL parameter name for ISBN.
pub const ISBN_URL_PARAM_NAME: &str = "isbn";

/// The domain name that is allowed to use the ID token.
/// Normally it would be our own domain name where all the server functions are hosted.
pub const TRUSTED_URLS: &str = "https://bookwormfood.com";

/// URL of sync.html lambda function.
pub const SYNC_HTML_URL: &str = "https://bookwormfood.com/sync.html";

/// Name of DDB table with the list of books per user
pub const USER_BOOKS_TABLE_NAME: &str = "user_books";

/// Name of S3 bucket for storing user photos
/// Value: `bookwormfood.com`.
pub const USER_PHOTOS_BUCKET_NAME: &str = "bookwormfood.com";
/// The path within the bucket where the user photos are stored.
/// Must include trailing slash.
/// Value: `photos/`.
pub const USER_PHOTOS_S3_PREFIX: &str = "photos/";

/// The file type of the user photos: .jpg
pub const USER_PHOTOS_S3_SUFFIX: &str = ".jpg";

/// Generates a user ID from the email by hashing it with a salt value.
/// The salt value is not secret and is used to prevent use of rainbow tables.
/// The user id may be used in URLs.
pub fn generate_user_id(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update("bookwormfood");
    hasher.update(email);
    hex::encode(hasher.finalize())
}

/// Where the reader is with the book.
/// Defaults to None.
#[wasm_bindgen]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum ReadStatus {
    ToRead = 0,
    Read = 1,
    Liked = 2,
}

impl std::fmt::Display for ReadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReadStatus::ToRead => write!(f, "ToRead"),
            ReadStatus::Read => write!(f, "Read"),
            ReadStatus::Liked => write!(f, "Liked"),
        }
    }
}

impl FromStr for ReadStatus {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ToRead" => Ok(ReadStatus::ToRead),
            "Read" => Ok(ReadStatus::Read),
            "Liked" => Ok(ReadStatus::Liked),
            _ => Err(()),
        }
    }
}

/// An internal representation of a book record.
/// Stored in the local storage and in the cloud.
/// This struct does not Default implementation to force thinking what attributes go where.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    /// This ISBN may differ from the key in the local storage or the industry IDs in the Google Books API.
    #[serde(default)]
    pub isbn: u64,
    /// When the book was last updated.
    #[serde(default)]
    pub timestamp_update: DateTime<Utc>,
    /// When the book was last sync'd.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_sync: Option<DateTime<Utc>>,
    /// Reading status, where the reader is with the book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_status: Option<ReadStatus>,
    /// The cover image URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    /// The book details from Google Books API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_info: Option<VolumeInfo>,
    /// A list of URLs for user-uploaded photos of the book.
    /// The list is sorted by the timestamp of the photo in the chronological order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<String>>,
    /// Dummy field to prevent struct instantiation without ISBN.
    #[serde(default, skip)]
    _dummy: usize,
}

impl Book {
    /// A fake ISBN to be used in cases where the real ISBN is not available.
    pub const FAKE_ISBN: u64 = 9700000000000;

    /// A very naive check if ISBN is a 10 or 13 digit number and has 97* prefix.
    /// TODO: changed it to a regex check.
    pub fn is_valid_isbn(isbn: &str) -> bool {
        ((isbn.len() == 13 && isbn.starts_with("97")) || isbn.len() == 10) && isbn.parse::<u64>().is_ok()
    }

    /// Updates the sync timestamp to the current time
    /// and returns the updated Self.
    pub fn with_new_sync_timestamp(self) -> Self {
        let mut book = self;
        book.timestamp_sync = Some(Utc::now());
        book
    }

    /// Reset the sync timestamp to None because the book failed to sync
    /// and returns the updated Self.
    pub fn without_sync_timestamp(self) -> Self {
        let mut book = self;
        book.timestamp_sync = None;
        book
    }

    /// Sets ISBN and timestamp_update=now fields.
    /// Use ::is_valid_isbn() to validate the value.
    pub fn new(isbn: u64) -> Self {
        Book {
            isbn,
            timestamp_update: Utc::now(),
            timestamp_sync: None,
            read_status: None,
            cover: None,
            title: None,
            authors: None,
            volume_info: None,
            photos: None,
            _dummy: 0,
        }
    }

    /// Returns true if title, authors or vol info are missing
    pub fn needs_enhancing(&self) -> bool {
        self.title.is_none() || self.authors.is_none() || self.volume_info.is_none()
    }
}

/// A list of book records.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Books {
    pub books: Vec<Book>,
}

impl Books {
    /// Sort the list of books by the timestamp of the last update - the latest update comes first.
    pub fn sort(&mut self) {
        self.books.sort_by(|a, b| b.timestamp_update.cmp(&a.timestamp_update));
    }
}
