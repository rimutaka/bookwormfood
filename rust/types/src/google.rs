/// Logic for fetching book data from Google Books API
///
/// Volume search by ISBN:
/// - https://www.googleapis.com/books/v1/volumes?q=isbn:9781761186769
///
/// Response: isbn_wasm_mod/data-samples/google-books-volume.json
///
/// API Reference: https://developers.google.com/books/docs/v1/reference/volumes#resource
///
use serde::{Deserialize, Serialize};

// /// Part of GoogleBooks API response
// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct IndustryIdentifier {
//     pub r#type: String,
//     pub identifier: String,
// }

/// Part of GoogleBooks API response
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageLinks {
    /// ~80 pixels wide
    pub small_thumbnail: Option<String>,
    /// ~128 pixels wide
    pub thumbnail: Option<String>,
    /// ~300 pixels wide
    pub small: Option<String>,
    /// ~575 pixels wide
    pub medium: Option<String>,
    /// ~800 pixels wide
    pub large: Option<String>,
    /// ~1280 pixels wide
    pub extra_large: Option<String>,
}

// /// Part of GoogleBooks API response
// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct ListPrice {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub amount: Option<f64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub currency_code: Option<String>,
// }

// /// Part of GoogleBooks API response
// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct SaleInfo {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub list_price: Option<ListPrice>,
// }

/// Part of GoogleBooks API response
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    pub title: String,
    #[serde(default = "Vec::new")]
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    // #[serde(default = "Vec::new")]
    // pub industry_identifiers: Vec<IndustryIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    // #[serde(default = "Vec::new")]
    // pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_links: Option<ImageLinks>,
}

/// Part of GoogleBooks API response
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub id: String,
    pub self_link: String,
    pub volume_info: VolumeInfo,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sale_info: Option<SaleInfo>,
}

impl VolumeInfo {
    /// Returns the best fitting thumbnail for the given max_width out of what is available.
    /// The result may not be optimal if no suitable size is available.
    /// Prefers a larger image over a smaller one.
    ///
    /// Returns the largest image if max_width is None.
    pub fn get_thumbnail(&self, max_width: Option<i32>) -> Option<String> {
        let image_links = match &self.image_links {
            Some(v) => v,
            None => return None,
        };

        // find the largest image available
        let largest = match image_links {
            ImageLinks {
                extra_large: Some(v), ..
            } => Some(v),
            ImageLinks { large: Some(v), .. } => Some(v),
            ImageLinks { medium: Some(v), .. } => Some(v),
            ImageLinks { small: Some(v), .. } => Some(v),
            ImageLinks { thumbnail: Some(v), .. } => Some(v),
            ImageLinks {
                small_thumbnail: Some(v),
                ..
            } => Some(v),
            _ => None,
        };

        // return the largest image if no max width was given
        let max_width = match max_width {
            Some(v) => v,
            None => return largest.cloned(),
        };

        // find the smallest image that is larger than or equal to max_width
        let larger_than_or_eq_max_width = match image_links {
            ImageLinks {
                small_thumbnail: Some(v),
                ..
            } if max_width <= 80 => Some(v),
            ImageLinks { thumbnail: Some(v), .. } if max_width <= 128 => Some(v),
            ImageLinks { small: Some(v), .. } if max_width <= 300 => Some(v),
            ImageLinks { medium: Some(v), .. } if max_width <= 575 => Some(v),
            ImageLinks { large: Some(v), .. } if max_width <= 800 => Some(v),
            ImageLinks {
                extra_large: Some(v), ..
            } => Some(v),
            _ => None,
        };

        // prefer the next one up, fall back to the largest available
        match (larger_than_or_eq_max_width, largest) {
            (Some(v), _) => Some(v.clone()),
            (_, Some(v)) => Some(v.clone()),
            _ => None,
        }
    }
}

/// The root of GoogleBooks API response
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volumes {
    pub kind: String,
    pub total_items: i64,
    #[serde(default = "Vec::new")]
    pub items: Vec<Volume>,
}
