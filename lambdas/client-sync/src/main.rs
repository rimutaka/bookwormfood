use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
/// This is a basic lambda for testing the emulator locally.
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
// use serde::{Deserialize, Serialize};
// use serde_json::from_str;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
// use wasm_mod::google;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_ansi(true)
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

    if let Some(method) = event.payload.request_context.http.method {
        info!("Method: {}", method);
        if method == "OPTIONS" {
            let mut headers = HeaderMap::new();
            headers.append("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
            headers.append("Access-Control-Allow-Methods", HeaderValue::from_static("GET, OPTIONS"));
            headers.append(
                "Access-Control-Allow-Headers",
                HeaderValue::from_static("auth0,authorization"),
            );

            return Ok(LambdaFunctionUrlResponse {
                status_code: 200,
                headers,
                cookies: Default::default(),
                body: Some("Hello".to_string()),
                is_base64_encoded: false,
            });
        }
    }

    // get bearer token from the event
    let authorization = match event.payload.headers.get("authorization") {
        Some(v) => v.to_str().unwrap_or("").to_string(),
        None => String::new(),
    };
    let auth0 = match event.payload.headers.get("auth0") {
        Some(v) => v.to_str().unwrap_or("").to_string(),
        None => String::new(),
    };

    info!("Authorization: {authorization}");
    info!("Auth0: {auth0}");

    // get index.html from S3
    let body = "Hello from client-sync".to_string();

    // create headers
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    // prepare the response
    let resp = LambdaFunctionUrlResponse {
        status_code: 200,
        headers,
        cookies: Default::default(),
        body: Some(body),
        is_base64_encoded: false,
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
