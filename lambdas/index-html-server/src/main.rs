use aws_lambda_events::{
    apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
    encodings::Body,
    http::{HeaderMap, HeaderValue},
};
/// This is a basic lambda for testing the emulator locally.
use lambda_runtime::{service_fn, Error, LambdaEvent};
// use serde::{Deserialize, Serialize};
// use serde_json::from_str;
use index::get_index_from_s3;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use wasm_mod::google;

mod index;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_ansi(true)
        .without_time()
        .with_max_level(LevelFilter::INFO)
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Received event: {:?}", event);

    let isbn = get_isbn_from_event(&event.payload);
    info!("ISBN: {isbn}");

    if isbn.is_empty() {
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", HeaderValue::from_static("text/text; charset=utf-8"));
        return Ok(ApiGatewayProxyResponse {
            status_code: 400,
            headers,
            multi_value_headers: Default::default(),
            body: Some(Body::Text("Invalid ISBN".to_string())),
            is_base64_encoded: false,
        });
    }
    // hello_lambda();

    let response = get_book_data(&isbn).await?;

    if response.items.is_empty() {
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", HeaderValue::from_static("text/text; charset=utf-8"));
        return Ok(ApiGatewayProxyResponse {
            status_code: 404,
            headers,
            multi_value_headers: Default::default(),
            body: Some(Body::Text("Book not found".to_string())),
            is_base64_encoded: false,
        });
    };

    // get index.html from S3
    let body = get_index_from_s3().await?;

    // get the thumbnail or use an empty string to keep the default provided in the HTML
    let thumb = match &response.items[0].volume_info.image_links {
        Some(v) => v.thumbnail.replace("http://", "https://"),
        None => "".to_string(),
    };

    // replace the title and ogImage in the HTML
    let body = replace_with_regex(&body, &response.items[0].volume_info.title, &thumb)?;

    // create a `Body` from the HTML string
    let body = Body::Text(body);

    // create headers
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    // prepare the response
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers,
        multi_value_headers: Default::default(),
        body: Some(body),
        is_base64_encoded: false,
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

/// Extracts the 13 digit ISBN from anywhere in the request path.
fn get_isbn_from_event(event: &ApiGatewayProxyRequest) -> String {
    // no path, no ISBN
    let path = match &event.path {
        Some(v) => v,
        None => {
            return String::new();
        }
    };

    // compile the regex
    let regex = match regex::Regex::new(r"/(\d{13})/") {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid ISBN regex. It's a bug. {:?}", e);
            return String::new();
        }
    };

    // get all matches
    let isbn = match regex.captures(path) {
        Some(v) => v,
        None => {
            error!("Request URL does not contain an ISBN. Path: {:?}", event.path);
            return String::new();
        }
    };

    // return the right capture group or a blank string
    match isbn.get(1) {
        Some(v) => v.as_str().to_string(),
        None => {
            error!("Request URL does not contain an ISBN. Path: {:?}", event.path);
            String::new()
        }
    }
}

/// Fetches book data from Google Books API. Shouldn't panic.
pub(crate) async fn get_book_data(isbn: &str) -> Result<google::Volumes, Error> {
    let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");

    let volumes = match reqwest::get(&url).await {
        Ok(v) => match v.json::<google::Volumes>().await {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::from(format!("Failed to convert book data into struct. {:?}", e)));
            }
        },
        Err(e) => {
            return Err(Error::from(format!("Failed to get book data from {isbn}. {:?}", e)));
        }
    };

    Ok(volumes)
}

/// Replaces Title and ogImage in the HTML, if the new values are not empty strings.
/// Otherwise keeps the existing values.
fn replace_with_regex(source: &str, title: &str, thumb: &str) -> Result<String, Error> {
    // use Cow to minimize allocations
    let replaced = std::borrow::Cow::Borrowed(source);

    // replace the title if the value is not empty
    // <title>📖📚📚</title>
    let replaced = if title.is_empty() {
        replaced
    } else {
        match regex::Regex::new(r"(<title>)([^<]+)") {
            Ok(v) => v.replace(source, ["${1}", title].concat()),
            Err(e) => {
                error!("Invalid title regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid title replacement regex"));
            }
        }
    };

    // replace the ogImage if the value is not empty
    // <meta id="ogImage" property="og:image" content="/img/og-thumb.png" />
    let replaced = if thumb.is_empty() {
        replaced
    } else {
        match regex::Regex::new(r#"(id="ogImage"[^>]+content=")([^"]+)"#) {
            Ok(v) => v.replace(&replaced, ["${1}", thumb].concat()),
            Err(e) => {
                error!("Invalid ogImage regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid ogImage replacement regex"));
            }
        }
    };

    Ok(replaced.to_string())
}
