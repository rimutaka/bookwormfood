use crate::{Result, RetryAfter};
use bookworm_types::IdToken;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

/// The name of the authorisation header containing the ID token with the user email.
pub const AUTH_HEADER: &str = "x-books-authorization";
/// The domain name that is allowed to use the ID token.
/// Normally it would be our own domain name where all the server functions are hosted.
pub const TRUSTED_URLS: &str = "https://bookworm.im";

/// The HTTP methods that are supported by the app.
pub(crate) enum HttpMethod<P> {
    Get,
    Post(P),
    Put(P),
    Delete,
}

/// Prepares and executes an HTTP request.
/// ## Types
/// * R - Response type, always required
/// * P - Payload type, may be omitted
/// ## Request types
/// * GET - if no payload is provided
/// * POST - if payload is provided
///
/// Do not include the id_token for URLs other than our own server side.
pub(super) async fn execute_http_request<P, R>(
    url: &str,
    method: HttpMethod<P>,
    runtime: &Window,
    id_token: &Option<IdToken>,
) -> Result<Option<R>>
where
    P: serde::Serialize,
    R: for<'de> serde::Deserialize<'de>,
{
    // log!("execute_get_request entered");

    // check if the target URL is for the bookworm domain and reset the token if it is not
    // ideally, this function should not even get the token if the URL is not trusted
    // it's an additional safety measure
    let id_token = if url.starts_with(TRUSTED_URLS) {
        id_token
    } else {
        if id_token.is_some() {
            log!("Token reset for untrusted URL. It's a bug.");
        }
        &None
    };

    // set request params
    let opts = RequestInit::new();
    opts.set_mode(RequestMode::Cors);

    // set HTTP method, add the payload and get a copy of it for later
    let payload = match &method {
        HttpMethod::Get => {
            opts.set_method("GET");
            None
        }
        HttpMethod::Post(v) => {
            opts.set_method("POST");
            match serde_json::to_string(v) {
                Ok(v) => {
                    // log!("Payload: {v}");
                    opts.set_body(&wasm_bindgen::JsValue::from_str(&v));
                    Some(v)
                }
                Err(e) => {
                    log!("Failed to serialize POST payload");
                    log!("{:?}", e);
                    // TODO: may be worth a retry
                    return Err(RetryAfter::Never);
                }
            }
        }
        HttpMethod::Put(v) => {
            opts.set_method("PUT");
            match serde_json::to_string(v) {
                Ok(v) => {
                    // log!("Payload: {v}");
                    opts.set_body(&wasm_bindgen::JsValue::from_str(&v));
                    Some(v)
                }
                Err(e) => {
                    log!("Failed to serialize PUT payload");
                    log!("{:?}", e);
                    // TODO: may be worth a retry
                    return Err(RetryAfter::Never);
                }
            }
        }
        HttpMethod::Delete => {
            opts.set_method("DELETE");
            None
        }
    };

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

    // set the auth header if the token is provided and the target is bookworm domain
    if let Some(id_token) = id_token {
        let _ = request.headers().set(AUTH_HEADER, id_token);
    }

    // payload-related headers
    if let Some(payload) = payload {
        // only set the content type if there is POST payload
        let _ = request.headers().set("content-type", "application/json");

        // calculate the SHA256 hash of the payload and set the header
        // needed for the CloudFront signed URLs
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let result = hasher.finalize();
        let result = hex::encode(result);
        // log!("X-Amz-Content-Sha256: {}", result);
        let _ = request.headers().set("X-Amz-Content-Sha256", &result);
    }

    // send the request and wait for the response
    let resp = JsFuture::from(runtime.fetch_with_request(&request)).await;

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

    // return an error if the status is anything but success
    let status = resp.status();
    log!("HTTP status: {status}");
    if !(200..300).contains(&status) {
        log!("HTTP request failed: {:?}", resp);
        return Err(RetryAfter::Never);
    }

    // return success if there is no response body
    if status == 204 {
        return Ok(None);
    }

    // Read the response stream to completion.
    // In theory, the stream may still be open and the op may take some time to complete
    let resp_body = match resp.json() {
        Ok(v) => JsFuture::from(v).await,
        Err(e) => {
            log!("Cannot convert response to Future for {url}: {:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as Response JSON:");
    // log!("resp: {:?}", resp);
    // log!("body: {:?}", resp_body);
    // log!("HTTP request completed");

    // Unwrap the response and handle the error
    let resp_body = match resp_body {
        Ok(v) => v,
        Err(e) => {
            log!("HTTP request failed: {url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as string:");
    // log!("{:?}", resp);

    // return a rust struct
    match serde_wasm_bindgen::from_value::<R>(resp_body) {
        Ok(v) => Ok(Some(v)),
        Err(e) => {
            log!("Cannot deser HTTP response into rust struct");
            log!("{url}");
            log!("{:?}", e);
            Err(RetryAfter::Never)
        }
    }
}
