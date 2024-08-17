use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use wasm_mod::AUTH_HEADER;

mod jwt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(LevelFilter::INFO)
        .init();

    let func = service_fn(my_handler);
    let runtime = Runtime::new(func);
    #[cfg(not(debug_assertions))]
    let runtime = runtime.layer(lambda_runtime::layers::TracingLayer::new());
    runtime.run().await?;
    Ok(())
}

pub(crate) async fn my_handler(
    event: LambdaEvent<LambdaFunctionUrlRequest>,
) -> Result<LambdaFunctionUrlResponse, Error> {
    // info!("Received event: {:?}", event);
    let path = event.payload.raw_path.clone().unwrap_or("".to_string());
    info!("Path: {}", path);

    // a collector for all headers added along the way
    let headers = HeaderMap::new();

    // get bearer token from the event
    let authorization = match event.payload.headers.get(AUTH_HEADER) {
        Some(v) => v.to_str().unwrap_or("").to_string(),
        None => String::new(),
    };

    info!("{:?}", event.payload.body);

    // info!("Auth: {authorization}");
    // info!("Headers: {:?}", event.payload.headers);

    // exit if no valid email is provided
    let email = match jwt::get_email(&authorization) {
        Ok(v) => v,
        Err(e) => {
            return Ok(LambdaFunctionUrlResponse {
                status_code: 403,
                headers: content_type_text(headers),
                cookies: Default::default(),
                body: Some(format!("Unauthorized: {:?}", e)),
                is_base64_encoded: false,
            });
        }
    };

    info!("Email: {:?}", email);

    let body = "Hello from client-sync".to_string();

    // prepare the response
    let resp = LambdaFunctionUrlResponse {
        status_code: 200,
        headers: content_type_text(headers),
        cookies: Default::default(),
        body: Some(body),
        is_base64_encoded: false,
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

/// A shortcut for adding `Content-Type: text/html ...` to the headers.
fn content_type_text(mut headers: HeaderMap) -> HeaderMap {
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));
    headers
}
