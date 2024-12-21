use crate::book;
use crate::http_req::{execute_http_request, HttpMethod};
use crate::utils::{log, upload_file};
use anyhow::Result;
use bookworm_types::{Book, IdToken, SHARED_PHOTOS_ENDPOINT_URL, SYNC_HTML_ENDPOINT_URL};
use web_sys::{FileList, Window};

/// Uploads a jpg file to S3 and updates the book record with a note about the upload.
/// Logs errors and returns a UI-friendly error if any of the steps fail.
pub(crate) async fn upload(runtime: &Window, isbn: u64, files: FileList, id_token: &Option<IdToken>) -> Option<Book> {
    // check if there is a file to upload
    if files.length() == 0 {
        log!("No files to upload");
        return None;
    }

    // the book record must exist in the local storage
    let mut book = match crate::book::get(runtime, isbn).await {
        Ok(Some(v)) => v,
        _ => {
            log!("Cannot get {isbn} record from local storage");
            return None;
        }
    };

    // there should be only one file for now, but the logic allows uploading multiple files in sequence
    for i in 0..files.length() {
        // get a signed URL from the Lambda for uploading to S3 directly
        // the file name is generated by the Lambda
        let signed_url = match execute_http_request::<Book, String>(
            SYNC_HTML_ENDPOINT_URL,
            HttpMethod::Put(book.clone()),
            runtime,
            id_token,
        )
        .await
        {
            Ok(Some(v)) if !v.is_empty() => v,
            // it is unlikely that a retry would help
            Err(e) => {
                log!("Failed to get signed URL for {isbn}: {:?}", e);
                return None;
            }
            _ => {
                log!("Lambda returned empty signed URL for {isbn}");
                return None;
            }
        };

        log!("Signed URL: {signed_url}");

        if let Some(file) = files.item(i) {
            log!("Uploading file: {} / {} bytes", file.name(), file.size());

            // upload the file to S3
            let status = upload_file(&signed_url, file).await;
            log!("Upload status: {status}");

            if status == 200 {
                log!("File uploaded successfully");
                let photo_id = match get_photo_id_from_presigned_url(&signed_url) {
                    Ok(v) => v,
                    Err(e) => {
                        log!(
                            "Failed to extract photo ID from the signed URL: {:?} / {}",
                            e,
                            signed_url
                        );
                        return None;
                    }
                };
                book = book.without_sync_timestamp().with_new_photo(photo_id);

                match book::save(&book, runtime).await {
                    Ok(v) => v,
                    Err(_) => {
                        return None;
                    }
                };
            } else {
                log!("Failed to upload file");
                return None;
            }
        };
    }

    Some(book)
}

/// Returns a list of URLs for the shared photos.
/// Logs errors and returns an empty list on failure.
pub(crate) async fn get_shared_photo_urls(runtime: &Window, share_id: &str, isbn: u64) -> Vec<String> {
    // check if the share ID is valid
    if share_id.is_empty() {
        log!("Empty share ID: {share_id}");
        return Vec::new();
    }

    // build the URL for the Lambda request
    let request_url = [
        SHARED_PHOTOS_ENDPOINT_URL,
        "?share_id=",
        share_id,
        "&isbn=",
        &isbn.to_string(),
    ]
    .concat();

    log!("Photos share request URL: {request_url}");

    // get the list of photo URLs from the Lambda
    match execute_http_request::<(), Vec<String>>(&request_url, HttpMethod::Get, runtime, &None).await {
        Ok(Some(v)) if !v.is_empty() => {
            // log!("Photo URLs: {:?}", v);
            v
        }
        // it is unlikely that a retry would help
        Err(e) => {
            log!("Failed to get photo URLs for {isbn} / {share_id}: {:?}", e);
            Vec::new()
        }
        _ => {
            log!("Lambda returned empty list of photo URLs for {isbn} / {share_id}");
            Vec::new()
        }
    }
}

/// Extracts the photo ID from a presigned URL.
/// URL example:
/// https://s3.us-east-1.amazonaws.com/bookwormfood.com/photos/2be54aceee5f0f64203861eee7938f594bb0304d84cab1583a0032dec8dcb80d-9780143107712-1727129470.jpg?x-id=PutObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=ASIAXORZMZKEHELYM43R%2F20240923%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240923T221110Z&X-Amz-Expires=300&X-Amz-SignedHeaders=host&X-Amz-Signature=8aabb59f938b19dacd229cef4d8913eb9077ebabf72158b22fec879ed925cd4f&X-Amz-Security-Token=IQoJb3JpZ2luX2VjEI%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCXVzLWVhc3QtMSJGMEQCIALfb811Irwf%2FoYuc6TfFdIHSbL7HdXn0S1izQgQ0nGnAiBVmFy%2BtpNhXrsc%2Fb4eCRuwfG%2BdkQIH9%2Fz%2FvqVhnvquSSrzAgjH%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F8BEAMaDDUxMjI5NTIyNTk5MiIMqXqpz4uIv9Toz1vvKscCT30Uco7kb8nk6z2h6qOSgejf399XrU1Iq2vInGksOoU8jTh%2FGxoM3P2Wm7CJwak342PySOGntqEbnlLZeB2AsM6LuN8gh5PgNvMCWTF%2FUEW8%2B3g%2FybWmXqFKyyimdRfY%2Bd5w8BjSMOtQ8iA43GAsi4gqKT2LkDCKyRowbvMaLQngHIMPrOLUvOOKUP4WDd8yS%2BFosdf02DzeJ92vUGPLmB34%2FkIebNUxwAxfZxEFYtoyEol%2FF2ATEy5WaxJO7H1820CjeqDGJ0Ve90eEbKOjAf3umI46j0hjYOI%2Ftbdjr7OWEaiXywr5A6tH8Xjt3vXyLGqf8Gi1jpD%2F6DhcFJFf7I9T6wzzIfWuWIzsF94R0DNrUL7SfPQ4Gj10xbGJijlbiKs0iptAj2mIbSVisb0be3LUzVwbk1ubfEuuFUkjjwQ4%2FRlzscWCMOXOx7cGOp8By6DMeJCalUeG2BXIUyfYE2hfzbfLRtYOj3TjSLJ8ER9mfb56UG27vj8I9ualCUSfEMH2FdCoQowO%2F5iKANWDUGWvLL2VbtFhIJpcsCl7i%2BIpleYl7VpQf1Nav%2BB7B%2BhI2OBQttHlcMDU2boKP0d7Q6ektHL4W9Y6IWE7CI5SVqH%2F%2BEqnuTztoxPzRhri5tjDAiMLL%2Flspz%2BJjakFCn9Q
fn get_photo_id_from_presigned_url(url: &str) -> Result<String> {
    let end = url
        .find(".jpg?")
        .ok_or_else(|| anyhow::Error::msg("Invalid presigned URL: missing .jpg?"))?;

    let start = url[..end]
        .rfind('-')
        .ok_or_else(|| anyhow::Error::msg("Invalid presigned URL: missing -"))?;

    let photo_id = url[start + 1..end].to_string();
    // log!("Photo ID: {photo_id}");

    Ok(photo_id)
}
