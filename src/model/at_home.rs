use std::path::PathBuf;

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

use crate::uuid::ChapterId;

use super::Image;

fn default_datetime() -> DateTime<Local> {
    Local::now() + Duration::minutes(15)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtHome {
    /// Date and Time when the images are no longer valid
    #[serde(skip, default = "default_datetime")]
    pub expires: DateTime<Local>,

    /// BaseUrl to construct final image URLs from.
    ///
    /// Url returned is valid for the requested chapter only, and for the duration of 15 minutes
    /// from the time of the response.
    pub base_url: String,
    pub chapter: AtHomeChapter,
}

impl AtHome {
    /// Get the full URLs for the full data images
    pub fn images(&self) -> Vec<Image> {
        self.chapter
            .data
            .iter()
            .map(|v| Image {
                url: format!("{}/data/{}/{v}", self.base_url, self.chapter.hash),
                expires: Some(self.expires),
                file_name: Some(PathBuf::from(v)),
            })
            .collect()
    }

    /// Get the full URLs for the data saver images
    pub fn saver_images(&self) -> Vec<Image> {
        self.chapter
            .data_saver
            .iter()
            .map(|v| Image {
                url: format!("{}/data-saver/{}/{v}", self.base_url, self.chapter.hash),
                expires: Some(self.expires),
                file_name: Some(PathBuf::from(v)),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeChapter {
    /// Chapter identifier
    pub hash: ChapterId,
    /// Full resolution images
    pub data: Vec<String>,
    /// Lower resolution images
    pub data_saver: Vec<String>,
}
