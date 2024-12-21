use anyhow::Error;
use aws_sdk_s3::{presigning::PresigningConfig, Client};
use bookworm_types::{Book, TIMESTAMP_BASE, USER_PHOTOS_BUCKET_NAME, USER_PHOTOS_S3_PREFIX, USER_PHOTOS_S3_SUFFIX};
use std::time::Duration;
use tracing::info;

/// Generates a presigned URL for uploading a photo of the book.
pub(crate) async fn get_signed_url(book: &Book, user_id: &str) -> Result<String, Error> {
    // join the different parts together with the path prefix
    let pid = [
        USER_PHOTOS_S3_PREFIX,
        user_id,
        "-",
        &book.isbn.to_string(),
        "-",
        // taking out the constant part of the timestamp makes it for a shorter URL
        // potentially fallible if NOW is in the past
        &(chrono::Utc::now().timestamp() as u64 - TIMESTAMP_BASE).to_string(),
        USER_PHOTOS_S3_SUFFIX,
    ]
    .concat();

    let client = Client::new(&aws_config::load_from_env().await);

    match client
        .put_object()
        .bucket(USER_PHOTOS_BUCKET_NAME)
        .key(pid)
        .presigned(PresigningConfig::expires_in(Duration::from_secs(300)).expect("Invalid duration. It's a bug."))
        .await
    {
        Ok(v) => {
            let url = v.uri().to_string();
            info!("Presigned URL: {}", url);
            Ok(url)
        }
        Err(e) => {
            info!(
                "Failed to generate presigned request {}/{}: {:?}",
                user_id, book.isbn, e
            );
            Err(Error::msg("Failed to generate presigned request".to_string()))
        }
    }
}
