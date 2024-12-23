use aws_sdk_s3::Client;
use lambda_runtime::Error;
use tracing::error;

/// Reads the index.html file from S3 and returns it as a string.
/// All errors are logged. Should not panic.
pub(crate) async fn get_index_from_s3() -> Result<String, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let bucket = std::env::var("BUCKET_NAME").unwrap_or_else(|_e| "bookworm-code".to_string());
    let response = match client
        .get_object()
        .bucket(bucket)
        .key("index.html".to_string())
        .send()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to get index.html from S3: {:?}", e);
            return Err(Error::from("Failed to get index.html from S3"));
        }
    };
    let body = match response.body.collect().await {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to read index.html bytes from S3: {:?}", e);
            return Err(Error::from("Failed to read index.html from S3"));
        }
    };

    let body = match String::from_utf8(body.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to convert index.html bytes into a UTF-8 string: {:?}", e);
            return Err(Error::from("index.html has invalid valid UTF-8"));
        }
    };

    Ok(body)
}
