use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

/// Wraps the result into a struct for JS to tell success from errors.
/// The error is a text message to be logged in the console for now.
/// It will have to be a more structured error in the future.
pub type WasmResult<T> = std::result::Result<T, String>;

/// A shared container for all types of responses placed in their own fields.
/// There can only be one type of response at a time.
/// This is needed for easy identification of the response type in JS.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WasmResponse {
    // everything has to be boxed because of the potential large difference in size
    // between the different types of responses
    // the memory is allocated based on the largest struct
    /// A list of book records.
    LocalBooks(Box<Option<WasmResult<crate::models::books::Books>>>),
    /// A single book record.
    LocalBook(Box<Option<WasmResult<crate::models::book::Book>>>),
    /// Result of a deletion operation for the enclosed ISBN.
    Deleted(Box<Option<WasmResult<String>>>),
}

impl fmt::Display for WasmResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let resp = match serde_json::to_string(&self) {
            Ok(v) => v,
            Err(e) => {
                log!("Failed to serialize book data {:?}", e);
                return write!(f, "Failed to serialize book data in WasmResponse. {:?}", e);
            }
        };

        // log!("Book data from WasmResponse:");
        // log!("{resp}");

        write!(f, "{resp}")
    }
}

/// WASM responses are sent back to the UI thread via Messaging API.
/// They are packaged into a common structure with each data type in its own field.
/// See `WasmResult` and `WasmResponse` for more details.
/// This function a proxy for report_progress() in progress.js
/// that does the actual sending.
#[wasm_bindgen(module = "/src/progress.js")]
extern "C" {
    pub fn report_progress(msg: String);
}
