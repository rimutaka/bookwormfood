use crate::utils::log;
use anyhow::{bail, Result};
use bookworm_types::{Book, Books};
use web_sys::Window;

/// Returns a sorted array of all book records stored locally.
/// Errors are logged.
pub(crate) fn get(runtime: &Window) -> Result<Books> {
    // connect to the local storage
    let ls = match runtime.local_storage() {
        Ok(Some(v)) => v,
        Err(e) => {
            bail!("Failed to get local storage: {:?}", e);
        }
        _ => {
            bail!("Local storage not available (OK(None))");
        }
    };

    // get the total number of records
    let number_of_records = match ls.length() {
        Ok(v) => v,
        Err(e) => {
            bail!("Failed to get local storage length: {:?}", e);
        }
    };

    // init the books array to the max possible size
    let mut books = Vec::with_capacity(number_of_records.try_into().unwrap_or_else(|_e| {
        log!("Failed to convert local storage length {number_of_records} to usize. It's a bug.");
        0
    }));

    // get one key at a time (inefficient, but the best we have with Local Storage)
    for i in 0..number_of_records {
        // get the key by index
        let key = match ls.key(i) {
            Ok(Some(v)) => v,
            Ok(None) => {
                log!("Key {i} not found in local storage");
                continue;
            }
            Err(e) => {
                log!("Failed to get key {i} from local storage: {:?}", e);
                continue;
            }
        };

        // ignore non-ISBN keys
        if !key.starts_with("97") {
            log!("Non-ISBN key ignored: {key}");
            continue;
        }

        // get value by key
        let book = match ls.get_item(&key) {
            Ok(Some(v)) => v,
            Ok(None) => {
                log!("Value not found in local storage: {key}");
                continue;
            }
            Err(e) => {
                log!("Failed to get value from local storage for {key}: {:?}", e);
                continue;
            }
        };

        // log!("{book}");

        // parse the string value into a book record
        let book = match serde_json::from_str::<Book>(&book) {
            Ok(v) => v,
            Err(e) => {
                log!("Failed to parse local storage book record for {key}: {:?}", e);
                continue;
            }
        };

        // log!("{:?}", book);

        // ignore books with no titles because it is likely to be a corrupted record
        // from the format change or a bug
        // the user will have no benefit from such records
        // TODO: compare the ISBN inside Book and the key - they may differ
        // TODO: delete these ignored records
        match &book.title {
            Some(v) => {
                if v.is_empty() {
                    log!("Empty title for {key}");
                    continue;
                }
            }
            None => {
                log!("Empty title for {key}");
                continue;
            }
        }

        books.push(book);
    }

    // the items in the local storage are randomly sorted
    // sort the list to make the latest scanned book come first
    let mut books = Books { books };
    books.sort();

    Ok(books)
}
