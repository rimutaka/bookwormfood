use aws_lambda_events::{
    http::{method::Method, HeaderMap, HeaderValue},
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bookworm_types::{ISBN_URL_PARAM_NAME, SHARE_ID_URL_PARAM_NAME};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod share;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(LevelFilter::INFO)
        .with_ansi(false)
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

    // get the book's ISBN from the query string
    let isbn = match event.payload.query_string_parameters.get(ISBN_URL_PARAM_NAME) {
        Some(v) => match v.parse::<u64>() {
            Ok(n) => n,
            Err(e) => {
                info!("Invalid ISBN param: {}, err: {}", v, e);
                return handler_response(Some("Invalid ISBN param".to_string()), 400);
            }
        },
        None => {
            info!("Missing ISBN param.");
            info!("All params: {:?}", event.payload.query_string_parameters);
            return handler_response(Some("Missing ISBN param".to_string()), 400);
        }
    };

    let share_id = match event.payload.query_string_parameters.get(SHARE_ID_URL_PARAM_NAME) {
        Some(v) => v,
        None => {
            info!("Missing share_id param.");
            info!("All params: {:?}", event.payload.query_string_parameters);
            return handler_response(Some("Missing share_id param".to_string()), 400);
        }
    };

    // decide on the action depending on the HTTP method
    let method = match event.payload.request_context.http.method {
        Some(v) => {
            if let Ok(method) = Method::from_bytes(v.as_bytes()) {
                method
            } else {
                info!("Invalid HTTP method: {v}");
                return handler_response(Some("Invalid HTTP method".to_string()), 400);
            }
        }
        None => {
            info!("Missing HTTP method");
            return handler_response(Some("Missing HTTP method. It's a bug.".to_string()), 400);
        }
    };
    info!("Method: {}", method);

    match method {
        // return the list of photos for the given share / isbn
        Method::GET => {
            let photos = share::get_photo_share_urls(isbn, share_id).await;
            match serde_json::to_string(&photos) {
                Ok(v) => handler_response(Some(v), 200),
                Err(e) => {
                    info!(
                        "Failed to serialize books for share_id: {}, isbn: {}: {:?}",
                        share_id, isbn, e
                    );
                    handler_response(Some(e.to_string()), 400)
                }
            }
        }
        // unsupported method
        _ => handler_response(Some("Unsupported HTTP method".to_string()), 400),
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
