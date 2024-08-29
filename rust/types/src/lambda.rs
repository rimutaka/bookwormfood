pub const USER_BOOKS_TABLE_NAME: &str = "user_books";

/// A unique identifier for the user.
#[derive(Clone)]
pub struct Uid(pub String);
/// The user email address from the JWT.
pub struct Email(pub String);
