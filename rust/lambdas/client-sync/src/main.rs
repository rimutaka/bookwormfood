use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use aws_sdk_dynamodb::Client;
use bookwormfood_types::{Book, AUTH_HEADER};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod book;
mod jwt;

const USER_BOOKS_TABLE_NAME: &str = "user_books";

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

    // get bearer token from the event
    let authorization = match event.payload.headers.get(AUTH_HEADER) {
        Some(v) => v.to_str().unwrap_or("").to_string(),
        None => String::new(),
    };

    info!("{:?}", event.payload.body);

    // try to deser the body into a book
    let book = match &event.payload.body {
        Some(v) => match serde_json::from_str::<Book>(v) {
            Ok(v) => v,
            Err(e) => {
                info!("Failed to parse payload: {:?}", e);
                return handler_response(Some("Invalid payload. Expected DdbBook".to_string()), 400);
            }
        },
        None => {
            info!("Empty input");
            return handler_response(Some("Missing payload. Expected DdbBook".to_string()), 400);
        }
    };

    // info!("Auth: {authorization}");
    // info!("Headers: {:?}", event.payload.headers);

    // exit if no valid email is provided
    let email = match jwt::get_email(&authorization) {
        Ok(v) => v,
        Err(e) => {
            info!("Unauthorized via JWT: {:?}", e);
            return handler_response(Some("Unauthorized via JWT".to_string()), 403);
        }
    };
    info!("Email: {:?}", email);

    // save the book to the database
    let client = Client::new(&aws_config::load_from_env().await);

    match book::save(&book, &client, &email).await {
        Ok(_) => handler_response(Some("Book saved".to_string()), 200),
        Err(e) => handler_response(Some(e.to_string()), 400),
    }
}

/// A shortcut for returning the lambda response in the required format.
/// Always returns OK.
fn handler_response(body: Option<String>, status: i64) -> Result<LambdaFunctionUrlResponse, Error> {
    // a collector for all headers added along the way
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    Ok(LambdaFunctionUrlResponse {
        status_code: status,
        headers,
        cookies: Default::default(),
        body,
        is_base64_encoded: false,
    })
}
