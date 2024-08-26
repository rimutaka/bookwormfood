use anyhow::Error;
use aws_sdk_s3::{presigning::PresigningConfig, Client};
use bookwormfood_types::{Book, USER_PHOTOS_BUCKET_NAME, USER_PHOTOS_S3_PREFIX};
use sha2::{Digest, Sha256};
use std::time::Duration;
use tracing::info;

/// Generates a presigned URL for uploading a photo of the book.
pub(crate) async fn get_signed_url(book: &Book, uid: &str) -> Result<String, Error> {
    // the object name is [uid/email hash]-[isbn]-[uuid].jpg

    // hash the email
    let mut hasher = Sha256::new();
    hasher.update(uid);
    let uid_hash = hex::encode(hasher.finalize());

    // join the different parts together with the path prefix
    let pid = [
        USER_PHOTOS_S3_PREFIX,
        &uid_hash,
        "-",
        &book.isbn.to_string(),
        "-",
        &uuid::Uuid::new_v4().simple().to_string(),
        ".jpg",
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
            info!("Failed to generate presigned request {}/{}: {:?}", uid, book.isbn, e);
            Err(Error::msg("Failed to generate presigned request".to_string()))
        }
    }
}
