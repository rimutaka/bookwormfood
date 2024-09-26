pub const USER_BOOKS_TABLE_NAME: &str = "user_books";

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
}
