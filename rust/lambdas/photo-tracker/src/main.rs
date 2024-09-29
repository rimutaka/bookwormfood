use aws_lambda_events::s3::{S3Event, S3EventRecord};

use bookwormfood_types::{USER_PHOTOS_S3_PREFIX, USER_PHOTOS_S3_SUFFIX};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod photo;

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

pub(crate) async fn my_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    // info!("Received event: {:?}", event);
    for record in event.payload.records {
        // there should be only one record in the event
        process_record(record).await?;
    }

    Ok(())
}

async fn process_record(record: S3EventRecord) -> Result<(), Error> {
    // get the file name from the record
    let object_key = match record.s3.object.key {
        Some(v) => v,
        None => {
            return Err(anyhow::Error::msg("Missing object key in the event record").into());
        }
    };

    // e.g. 2be54aceee5f0f64203861eee7938f594bb0304d84cab1583a0032dec8dcb80d-9780143107712-1727064374.jpg
    info!("File name: {object_key}");

    // check if the file name starts with the right prefix and ends with .jpg and remove the prefix and suffix
    let ids = if object_key.starts_with(USER_PHOTOS_S3_PREFIX) && object_key.ends_with(USER_PHOTOS_S3_SUFFIX) {
        // strip_prefix returns none if there was no match - return the original string then
        // TODO: this is lame
        let object_key = object_key.strip_prefix(USER_PHOTOS_S3_PREFIX).unwrap_or(&object_key);
        object_key
            .strip_suffix(USER_PHOTOS_S3_SUFFIX)
            .unwrap_or(object_key)
            .split('-')
            .collect::<Vec<&str>>()
    } else {
        return Err(anyhow::Error::msg("Invalid file name: suffix or prefix mismatch").into());
    };

    // a valid file name consists of 3 parts
    if ids.len() != 3 {
        return Err(anyhow::Error::msg(format!("Invalid file name: expected 3 parts, got {}", ids.len())).into());
    }

    let user_id = ids[0].to_string();
    let isbn = ids[1].to_string();
    let photo_id = ids[2].to_string();
    info!(
        "Event:{:?}, UID: {}, ISBN: {}, Photo ID{}",
        record.event_name, user_id, isbn, photo_id
    );

    // save the photo ID to the user book record

    match &record.event_name {
        Some(v) if v.starts_with("ObjectCreated:") => photo::add_photo_to_ddb(&user_id, isbn, photo_id).await,
        Some(v) if v.starts_with("ObjectRemoved:") => photo::remove_photo_from_ddb(&user_id, isbn, photo_id).await,
        _ => {
            info!("Unhandled S3 event: {:?}", record.event_name);
            Ok(())
        }
    }
}
