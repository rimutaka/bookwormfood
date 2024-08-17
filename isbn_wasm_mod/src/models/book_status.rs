use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Where the reader is with the book.
/// Defaults to None.
#[wasm_bindgen]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum BookStatus {
    ToRead,
    Read,
    Liked,
}