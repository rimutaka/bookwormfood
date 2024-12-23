#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::filter::LevelFilter;

pub const USER_BOOKS_TABLE_NAME: &str = "user_books";

/// An index for finding the user ID for a particular share ID.
pub const USER_BOOKS_SHARE_INDEX_NAME: &str = "share-isbn-index";

/// The list of field names in `USER_BOOKS_TABLE_NAME` table.
pub mod user_books_table_fields {
    pub const AUTHORS: &str = "authors";
    /// The user email address from the JWT.
    /// Should never be returned to the caller.
    pub const EMAIL: &str = "email";
    /// Sort key: ISBN of the book.
    pub const ISBN: &str = "isbn";
    /// List of user uploaded photos for the book.
    pub const PHOTO_IDS: &str = "photo_ids";
    /// Where the reader is with the book.
    pub const READ_STATUS: &str = "read_status";
    pub const TITLE: &str = "title";
    /// Partition key: user ID.
    pub const UID: &str = "uid";
    /// When the record was last updated.
    pub const UPDATED: &str = "updated";
    /// A timestamp of the very first photo uploaded into the book
    /// `share` is a reserved keyword in DDB and must be escaped.
    pub const SHARE_ID: &str = "share";
}

/// Initializes the tracing subscriber for CloudWatch or local logging.
/// - CloudWatch: compact format with x-ray data at the end
/// - Local: no time, ANSI color
#[cfg(not(target_arch = "wasm32"))]
pub fn init_tracing_subscriber() {
        // this init is required to enable CloudWatch error logging by the runtime
        #[cfg(debug_assertions)] // a more compact format for local debugging
        tracing_subscriber::fmt()
            .without_time()
            .with_max_level(LevelFilter::INFO)
            .with_ansi(true)
            .init();
    
        #[cfg(not(debug_assertions))]
        tracing_subscriber::fmt() // CloudWatch-friendly format
            .with_max_level(LevelFilter::INFO)
            .with_ansi(false) 
            .compact() // puts x-ray data at the end
            .init();
}