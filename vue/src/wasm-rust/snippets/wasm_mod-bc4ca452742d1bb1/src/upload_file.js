// This function is imported into lib.rs as a simpler way of uploading a file to S3.
// Returns HTTP status that is converted into u32 in Rust.
// 0 is returned if an error occurs.
export async function upload_file_return_http_status(signed_url, file) {
  console.log(`Url: ${signed_url}, file: `, file);

  try {
  // Send the file to the server
    const response = await fetch(signed_url, {
      body: file,
      method: 'PUT',
      headers: {
        'Content-Type': 'image/jpg',
      },
      mode: 'cors',
      cache: 'no-cache',
    });

    // log and return the response status
    const status = response.status
    console.log(`File upload status: ${status}`);
    
    return status;

  } catch (error) {
    console.log("File upload error:", error);
    return 0;
  }
}