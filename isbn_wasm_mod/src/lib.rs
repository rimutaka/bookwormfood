use utils::{WasmResponse, WasmResult};
use wasm_bindgen::prelude::*;
use web_sys::Window;

pub mod google;
#[macro_use]
pub(crate) mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// /// Makes JS `console.log` available in Rust
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace=console)]
//    pub(crate) fn log(s: &str);
// }

// pub(crate) use log;

// /// Logs output into browser console. It is not the same console as for the web page because the extension runs separately.
// /// Look for the service worker console.
// macro_rules!  log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into())
//     }
// }

/// A demo function to test if WASM is callable from background.js
#[wasm_bindgen]
pub fn hello_wasm() {
    log!("Hello from WASM!");
}

pub fn hello_lambda() {
    log!("Hello from Lambda!");
}

/// A demo function to test if WASM is callable from background.js
#[wasm_bindgen]
pub fn log_isbn(isbn: String) {
    log!("WASM ISBN: {isbn}");
    report_progress(isbn);
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

/// The main entry point for the UI thread to request book data.
/// Multiple responses are sent back via `progress.js` to the UI thread.
/// See `fn report_progress()` for more details.
#[wasm_bindgen]
pub async fn get_book_data(isbn: String) {
    log!("Getting book data for ISBN: {isbn}");

    // need the runtime for the global context and fetch
    let runtime = match get_runtime().await {
        Ok(v) => v,

        // this would be a bug
        Err(e) => {
            log!("Failed to get runtime: {:?}", e);
            return;
        }
    };

    // GoogleBooks seems to have the most accurate and most up to data on all books
    // get it first, send back to the UI and use the bits of info from there to do more
    // fetching from other sources
    let resp = match google::get_book_data(&isbn, &runtime).await {
        Ok(v) => {
            log!("Book data received");
            WasmResponse {
                google_books: Some(WasmResult::Ok(v)),
            }
        }
        Err(e) => {
            log!("Failed to get book data");
            log!("{:?}", e);
            WasmResponse {
                google_books: Some(WasmResult::Err(format!("{:?}", e))),
            }
        }
    };

    // log!("Book data below:");
    // log!("{:?}", resp);

    report_progress(resp.to_string());
}

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

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Returns the right type of runtime (Window) for the current browser
/// or an error if the runtime is not available.
async fn get_runtime() -> std::result::Result<Window, &'static str> {
    // this is a fallback for Firefox, but it does not make sense why they would use Window in
    // web workers
    match web_sys::window() {
        Some(v) => {
            log!("Runtime Window found");
            Ok(v)
        }
        None => Err("Missing browser runtime. It's a bug."),
    }
}
