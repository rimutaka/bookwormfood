pub use book::{Book, ReadStatus};
use serde::{Deserialize, Serialize};

mod book;
pub mod google;
pub mod jwt;
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

pub type IdToken = String;

/// The name of the authorisation header containing the ID token with the user email.
pub const AUTH_HEADER: &str = "x-books-authorization";

/// Value: `isbn`. The URL parameter name for ISBN.
pub const ISBN_URL_PARAM_NAME: &str = "isbn";

/// The domain name that is allowed to use the ID token.
/// Normally it would be our own domain name where all the server functions are hosted.
pub const TRUSTED_URLS: &str = "https://bookwormfood.com";

/// URL of sync.html lambda function.
pub const SYNC_HTML_URL: &str = "https://bookwormfood.com/sync.html";

/// URL for fetching user photos for the front-end.
/// It should point at CloudFront with S3 as the origin.
pub const USER_PHOTOS_BASE_URL: &str = "https://bookwormfood.com/";

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

/// Timestamp for 1 Jan 2024.
/// All photo timestamps have this part subtracted because it is constant.
pub const TIMESTAMP_BASE:u64 = 1_704_067_200;

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

    /// Creates a leaner clone with some optional fields set to None
    /// to reduce the size of the JSON payload.
    pub fn lean_copy(&self) -> Books {
        Books {
            books: self
                .books
                .iter()
                .map(|v| {
                    let mut book = v.clone();
                    book.volume_info = None;
                    book.photos = None;
                    book.cover = None;
                    book.timestamp_sync = None;
                    book.share = None;

                    book
                })
                .collect(),
        }
    }
}
