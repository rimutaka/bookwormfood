use crate::google::VolumeInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Where the reader is with the book.
/// Defaults to None.
#[wasm_bindgen]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum ReadStatus {
    ToRead = 0,
    Read = 1,
    Liked = 2,
}

impl std::fmt::Display for ReadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReadStatus::ToRead => write!(f, "ToRead"),
            ReadStatus::Read => write!(f, "Read"),
            ReadStatus::Liked => write!(f, "Liked"),
        }
    }
}

impl FromStr for ReadStatus {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ToRead" => Ok(ReadStatus::ToRead),
            "Read" => Ok(ReadStatus::Read),
            "Liked" => Ok(ReadStatus::Liked),
            _ => Err(()),
        }
    }
}

/// An internal representation of a book record.
/// Stored in the local storage and in the cloud.
/// This struct does not Default implementation to force thinking what attributes go where.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    /// This ISBN may differ from the key in the local storage or the industry IDs in the Google Books API.
    #[serde(default)]
    pub isbn: u64,
    /// When the book was last updated.
    #[serde(default)]
    pub timestamp_update: DateTime<Utc>,
    /// When the book was last sync'd.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_sync: Option<DateTime<Utc>>,
    /// Reading status, where the reader is with the book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_status: Option<ReadStatus>,
    /// The cover image URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    /// The book details from Google Books API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_info: Option<VolumeInfo>,
    /// A list of URLs for user-uploaded photos of the book.
    /// The list is sorted by the timestamp of the photo in the chronological order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<String>>,
    /// A shortcode to access the book details for this user.
    /// It is set to the timestamp of the first photo and is never updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_id: Option<u64>,
    /// Dummy field to prevent struct instantiation without ISBN.
    #[serde(default, skip)]
    _dummy: usize,
}

impl Book {
    /// A fake ISBN to be used in cases where the real ISBN is not available.
    pub const FAKE_ISBN: u64 = 9700000000000;

    /// A very naive check if ISBN is a 10 or 13 digit number and has 97* prefix.
    /// TODO: changed it to a regex check.
    pub fn is_valid_isbn(isbn: &str) -> bool {
        ((isbn.len() == 13 && isbn.starts_with("97")) || isbn.len() == 10) && isbn.parse::<u64>().is_ok()
    }

    /// Updates the sync timestamp to the current time
    /// and returns the updated Self.
    pub fn with_new_sync_timestamp(self) -> Self {
        let mut book = self;
        book.timestamp_sync = Some(Utc::now());
        book
    }

    /// Reset the sync timestamp to None because the book failed to sync
    /// and returns the updated Self.
    pub fn without_sync_timestamp(self) -> Self {
        let mut book = self;
        book.timestamp_sync = None;
        book
    }

    /// Sets ISBN and timestamp_update=now fields.
    /// Use ::is_valid_isbn() to validate the value.
    pub fn new(isbn: u64) -> Self {
        Book {
            isbn,
            timestamp_update: Utc::now(),
            timestamp_sync: None,
            read_status: None,
            cover: None,
            title: None,
            authors: None,
            volume_info: None,
            photos: None,
            share_id: None,
            _dummy: 0,
        }
    }

    /// Adds a new photo to the list of photos and returns the updated Self.
    /// Photos are sorted by ID, which is a timestamp.
    /// The share field is set to the photo ID if it's the first uploaded photo
    /// and no share value exists.
    /// Share value can be overwritten by the value from the cloud.
    pub fn with_new_photo(self, photo_id: String) -> Self {
        // share can only be set once
        let share_id = match self.share_id {
            Some(v) => Some(v),
            None => match photo_id.parse::<u64>() {
                Ok(n) => Some(n),
                Err(_) => None,
            },
        };

        // add the photo to the list and sort in the chronological order
        let mut photos = self.photos.unwrap_or_default();
        photos.push(photo_id);
        photos.sort();

        Book {
            photos: Some(photos),
            share_id,
            ..self
        }
    }

    /// Adds a missing parts of the book that can be calculated from the existing data.
    /// E.g. it transforms photo IDs into URLs.
    pub fn hydrate(self, user_id: &str) -> Self {
        if let Some(photos) = self.photos {
            Book {
                photos: Some(Self::hydrate_photos(user_id, self.isbn, photos)),
                ..self
            }
        } else {
            // no photos - return the book as is
            self
        }
    }

    /// Returns a list of photo URLs for the user and the ISBN based on the photo IDs.
    /// e.g. https://bookwormfood.com/photos/8cbf509d254774a13ede02ce246d39434950c93aa328407e7fef657d2bb6f737-9780143107712-23520065.jpg
    pub fn hydrate_photos(user_id: &str, isbn: u64, photos: Vec<String>) -> Vec<String> {
        // build the front-end part of the URL
        let front_part = [
            crate::USER_PHOTOS_BASE_URL,
            crate::USER_PHOTOS_S3_PREFIX,
            user_id,
            "-",
            &isbn.to_string(),
            "-",
        ]
        .concat();

        // loop thru all photos to build the URLs
        photos
            .into_iter()
            .map(|v| [front_part.to_owned(), v, crate::USER_PHOTOS_S3_SUFFIX.to_owned()].concat())
            .collect()
    }

    /// Returns true if title, authors or vol info are missing
    pub fn needs_enhancing(&self) -> bool {
        self.title.is_none() || self.authors.is_none() || self.volume_info.is_none()
    }

    /// Copies Title and Authors from the cloud book if there is no local data.
    /// Copies the list of photos from the cloud.
    /// Uses the latest timestamp_update out of the two.
    /// Keeps the local status and all other details.
    pub fn merge_from_cloud(&mut self, other: &Self) {
        // compile a more complete version of the book
        // since book details come from the same source the precedence should be given to the local data

        self.timestamp_update = if self.timestamp_update > other.timestamp_update {
            self.timestamp_update
        } else {
            other.timestamp_update
        };

        if self.title.is_none() {
            self.title = other.title.clone()
        };
        if self.authors.is_none() {
            self.authors = other.authors.clone()
        };
        // photos in the cloud are always more authoritative than the local state
        self.photos = other.photos.clone();
        // this is set when the first photo is uploaded
        // the value persists even if the photo was deleted
        self.share_id = other.share_id;
    }
}
