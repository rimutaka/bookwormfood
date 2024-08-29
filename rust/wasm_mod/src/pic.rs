use crate::http_req::{execute_http_request, HttpMethod, IdToken};
use crate::utils::{log, upload_file};
use anyhow::{bail, Result};
use bookwormfood_types::{Book, SYNC_HTML_URL};
use web_sys::{FileList, Window};

/// Uploads a jpg file to S3 and updates the book record with a note about the upload.
/// Logs errors and returns a UI-friendly error if any of the steps fail.
pub(crate) async fn upload(runtime: &Window, isbn: u64, files: FileList, id_token: Option<IdToken>) -> Result<()> {
    // check if there is a file to upload
    if files.length() == 0 {
        log!("No files to upload");
        return Ok(());
    }

    // get the reference to the local storage
    // let ls = get_local_storage(runtime)?;

    // get signed URL for this ISBN
    let book = Book::new(isbn);

    let signed_url =
        match execute_http_request::<Book, String>(SYNC_HTML_URL, HttpMethod::Put(book), runtime, &id_token).await {
            Ok(Some(v)) if !v.is_empty() => v,
            Err(e) => {
                log!("Failed to get signed URL for {isbn}: {:?}", e);
                bail!("Failed to get signed URL");
            }
            _ => {
                log!("Lambda returned empty signed URL for {isbn}");
                bail!("Failed to get signed URL");
            }
        };

    log!("Signed URL: {signed_url}");

    // get file details
    //
    for i in 0..files.length() {
        if let Some(file) = files.item(i) {
            log!("File: {} / {}", file.name(), file.size());

            // upload the file to S3
            let status = upload_file(&signed_url, file).await;
            log!("Upload status: {status}");
        };
    }

    Ok(())
}
