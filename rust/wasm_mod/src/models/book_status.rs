use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
