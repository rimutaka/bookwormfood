use crate::{BrowserRuntime, Result, RetryAfter};
use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Logs output into browser console.
macro_rules!  log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}
// this line is needed to make the macro available to the rest of the module
pub(crate) use log;

/// Prepares and executes an HTTP request.
/// ## Types
/// * R - Response type, always required
/// * P - Payload type, may be omitted
/// ## Request types
/// * GET - if no payload is provided
/// * POST - if payload is provided
pub(super) async fn execute_http_request<R, P>(
    url: &str,
    payload: Option<&P>,
    runtime: &BrowserRuntime,
) -> Result<R>
where
    R: for<'de> serde::Deserialize<'de>,
    P: serde::Serialize,
{
    // log!("execute_get_request entered");
    // set request params
    let mut opts = RequestInit::new();
    opts.mode(RequestMode::Cors);
    match payload {
        Some(v) => {
            opts.method("POST");

            match serde_json::to_string(v) {
                Ok(v) => {
                    opts.body(Some(&wasm_bindgen::JsValue::from_str(&v)));
                }
                Err(e) => {
                    log!("Failed to serialize POST payload");
                    log!("{:?}", e);
                    // TODO: may be worth a retry
                    return Err(RetryAfter::Never);
                }
            }
        }
        None => {
            opts.method("GET");
        }
    }

    // log!("{url}");

    // create the request
    let request = match Request::new_with_str_and_init(url, &opts) {
        Ok(v) => v,
        Err(e) => {
            log!("HTTP Request creation failed");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Request created");

    // add headers
    let _ = request.headers().set("Accept", "application/json");
    if payload.is_some() {
        // only set the content type if there is POST payload
        let _ = request.headers().set("content-type", "application/json");
    }

    // log!("Headers set");

    // both window and globalscope have the same interface, but they are separate types so Rust has
    // to have separate paths for them
    // the output is the same type for both
    let resp = match runtime {
        BrowserRuntime::ChromeWorker(v) => JsFuture::from(v.fetch_with_request(&request)).await,
        BrowserRuntime::FireFoxWindow(v) => JsFuture::from(v.fetch_with_request(&request)).await,
    };

    // unwrap the response
    let resp = match resp {
        Ok(v) => v,
        Err(e) => {
            log!("HTTP request failed");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as JsValue:");
    // log!("{:?}", resp_value);

    // exit if the response is not of the expected type
    if !resp.is_instance_of::<Response>() {
        log!("HTTP response in not of type Response");
        log!("{url}");
        log!("{:?}", resp);
        // TODO: may be worth a retry
        return Err(RetryAfter::Never);
    };

    // this is unlikely to fail because of the previous type check
    let resp: Response = match resp.dyn_into() {
        Ok(v) => v,
        Err(e) => {
            log!("Cannot typecast response to Response");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as Response:");
    // log!("{:?}", resp);

    // Read the response stream to completion.
    // In theory, the stream may still be open and the op may take some time to complete
    let resp = match resp.json() {
        Ok(v) => JsFuture::from(v).await,
        Err(e) => {
            log!("Cannot convert Promise to Future");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as Response JSON:");
    // log!("{:?}", resp);

    // log!("HTTP request completed");

    // Unwrap the response and handle the error
    let resp = match resp {
        Ok(v) => v,
        Err(e) => {
            log!("Spotify request failed");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as string:");
    // log!("{:?}", resp);

    // convert into a rust struct
    let playlist = match serde_wasm_bindgen::from_value::<R>(resp) {
        Ok(v) => v,
        Err(e) => {
            log!("Cannot deser HTTP response into rust struct");
            log!("{url}");
            log!("{:?}", e);
            return Err(RetryAfter::Never);
        }
    };

    Ok(playlist)
}

/// Wraps the result into a struct for JS to tell success from errors.
/// The error is a text message to be logged in the console for now.
/// It will have to be a more structured error in the future.
pub(crate) type WasmResult<T> = std::result::Result<T, String>;

/// A shared container for all types of responses placed in their own fields.
/// There can only be one type of response at a time.
/// This is needed for easy identification of the response type in JS.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WasmResponse {
    pub google_books: Option<WasmResult<crate::google::Volumes>>,
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
