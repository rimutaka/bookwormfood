use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
/// This is a basic lambda for testing the emulator locally.
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
// use serde::{Deserialize, Serialize};
// use serde_json::from_str;
use index::get_index_from_s3;
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;
use bookworm_types::google;

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

    // get the ISBN from the event
    let isbn = get_isbn_from_event(&path);
    info!("ISBN: {isbn}");

    // get index.html from S3
    let body = get_index_from_s3().await?;

    // get book data from Google Books API
    let book_data = if isbn.is_empty() {
        None
    } else {
        get_book_data(&isbn).await
    };

    // replace the title and og:url in the HTML if book data is available
    // otherwise return unmodified index.html
    let body = match book_data {
        Some(v) => {
            // try to use the description from the book data
            let description = match &v.items[0].volume_info.description {
                Some(v) if !v.trim().is_empty() => v,
                _ => {
                    info!("Blank description");
                    "Information about this book, its author, places to buy or borrow"
                }
            };
            replace_with_regex(&body, &v.items[0].volume_info.title, description, &path)?
        }
        None => body,
    };

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

/// Extracts the 13 digit ISBN from anywhere in the request path.
fn get_isbn_from_event(path: &str) -> String {
    // no point looking for ISBN if there is no path
    if path.is_empty() || path == "/" {
        return String::new();
    }

    // compile the regex
    let regex = match regex::Regex::new(r"/(\d{13})\b") {
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
            info!("Request URL does not contain an ISBN. Path: {path}");
            return String::new();
        }
    };

    // return the right capture group or a blank string
    match isbn.get(1) {
        Some(v) => v.as_str().to_string(),
        None => {
            error!("Request URL does not contain an ISBN. Path: {path}");
            String::new()
        }
    }
}

/// Fetches book data from Google Books API.
/// Logs errors internally.
pub(crate) async fn get_book_data(isbn: &str) -> Option<google::Volumes> {
    let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");

    // get the data from Google Books API, but it's a list of search results
    // and there is no guarantee it finds the right book
    match reqwest::get(&url).await {
        Ok(v) => match v.json::<google::Volumes>().await {
            Ok(v) => Some(v),
            Err(e) => {
                error!("Failed to convert google books response into struct. {:?}", e);
                None
            }
        },
        Err(e) => {
            error!("Failed to get google books data for ISBN {isbn}. {:?}", e);
            None
        }
    }
}

/// Replaces Title and ogImage in the HTML, if the new values are not empty strings.
/// Otherwise keeps the existing values.
fn replace_with_regex(source: &str, title: &str, description: &str, path: &str) -> Result<String, Error> {
    // use Cow to minimize allocations
    let replaced = std::borrow::Cow::Borrowed(source);

    // replace the og:url if the value is not empty
    //   <meta property="og:url" content="http://bookworm.im">
    let replaced = if path.is_empty() || path == "/" {
        replaced
    } else {
        match regex::Regex::new(r#"("og:url"[^>]+content=")([^"]+)"#) {
            Ok(v) => v.replace(
                &replaced,
                ["${1}", &["https://bookworm.im", path].concat()].concat(),
            ),
            Err(e) => {
                error!("Invalid og:url regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid og:url replacement regex"));
            }
        }
    };

    // description should not be empty
    // we use a generic blurb if the book data has none
    // trip it to 500 characters
    let description = if description.len() > 500 {
        info!("Long description: {}", description.len());
        &description[..500]
    } else {
        description
    };

    // <meta name="description" content="A pocket assistant ...">
    let replaced = match regex::Regex::new(r#"("description"[^>]+content=")([^"]+)"#) {
        Ok(v) => v.replace(&replaced, ["${1}", description].concat()),
        Err(e) => {
            error!("Invalid description regex. It's a bug. {:?}", e);
            return Err(Error::from("Invalid description replacement regex"));
        }
    };

    // <meta property="og:description" content="A pocket assistant for keen readers...">
    let replaced = match regex::Regex::new(r#"("og:description"[^>]+content=")([^"]+)"#) {
        Ok(v) => v.replace(&replaced, ["${1}", description].concat()),
        Err(e) => {
            error!("Invalid og:description regex. It's a bug. {:?}", e);
            return Err(Error::from("Invalid og:description replacement regex"));
        }
    };

    // <meta name="twitter:description" content="A pocket assistant for keen readers..." />
    match regex::Regex::new(r#"("twitter:description"[^>]+content=")([^"]+)"#) {
        Ok(v) => v.replace(&replaced, ["${1}", description].concat()),
        Err(e) => {
            error!("Invalid twitter:description regex. It's a bug. {:?}", e);
            return Err(Error::from("Invalid description replacement regex"));
        }
    };

    // replace the title in multiple places if the value is not empty
    let replaced = if title.is_empty() {
        replaced.to_string()
    } else {
        // <title>📖📚📚</title>
        let replaced = match regex::Regex::new(r"(<title>)([^<]+)") {
            Ok(v) => v.replace(&replaced, ["${1}", title].concat()),
            Err(e) => {
                error!("Invalid title regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid title replacement regex"));
            }
        };

        // <meta property="og:title" content="Scan ISBN to record or share a book" />
        let replaced = match regex::Regex::new(r#"("og:title"[^>]+content=")([^"]+)"#) {
            Ok(v) => v.replace(&replaced, ["${1}", title].concat()),
            Err(e) => {
                error!("Invalid og:title regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid og:title replacement regex"));
            }
        };

        // <meta name="twitter:title" content="Scan ISBN to record or share a book" />
        let replaced = match regex::Regex::new(r#"("twitter:title"[^>]+content=")([^"]+)"#) {
            Ok(v) => v.replace(&replaced, ["${1}", title].concat()),
            Err(e) => {
                error!("Invalid twitter:title regex. It's a bug. {:?}", e);
                return Err(Error::from("Invalid twitter:title replacement regex"));
            }
        };

        replaced.to_string()
    };

    Ok(replaced)
}
