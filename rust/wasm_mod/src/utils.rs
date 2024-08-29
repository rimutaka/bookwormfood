use anyhow::{bail, Result};
use wasm_bindgen::prelude::*;
use web_sys::{File, Storage, Window};

/// Logs output into browser console.
macro_rules!  log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}
// this line is needed to make the macro available to the rest of the module
pub(crate) use log;

/// Returns the right type of runtime (Window) for the current browser
/// or an error if the runtime is not available.
pub(crate) async fn get_runtime() -> std::result::Result<Window, &'static str> {
    // this is a fallback for Firefox, but it does not make sense why they would use Window in
    // web workers
    match web_sys::window() {
        Some(v) => {
            // log!("Runtime Window found");
            Ok(v)
        }
        None => Err("Missing browser runtime. It's a bug."),
    }
}

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

/// A shorcut for getting the local storage.
/// TODO: put it inside OneCell, but it's probably not Send
pub(crate) fn get_local_storage(runtime: &Window) -> Result<Storage> {
    // connect to the local storage
    match runtime.local_storage() {
        Ok(Some(v)) => Ok(v),
        Err(e) => {
            bail!("Failed to get local storage: {:?}", e);
        }
        _ => {
            bail!("Local storage not available (OK(None))");
        }
    }
}

/// Uploads a file to S3 using a signed URL via an external JS function call.
/// Returns the HTTP status code of the upload or `0` if the upload failed with an error.
/// The error is logged in the console by the JS function.
pub(crate) async fn upload_file(signed_url: &str, file: File) -> u32 {
    let result = upload_file_return_http_status(signed_url, file).await;
    result.as_f64().unwrap_or(0.0).trunc() as u32
}

/// A JS-based file uploader using fetch+PUT for S3.
#[wasm_bindgen(module = "/src/upload_file.js")]
extern "C" {
    async fn upload_file_return_http_status(signed_url: &str, file: File) -> JsValue;
}
