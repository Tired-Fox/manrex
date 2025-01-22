use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AtHome {
    /// BaseUrl to construct final image URLs from.
    ///
    /// Url returned is valid for the requested chapter only, and for the duration of 15 minutes
    /// from the time of the response.
    pub base_url: String,
    pub chapter: AtHomeChapter
}

impl AtHome {
    /// Get the full URLs for the full data images
    pub fn images(&self) -> Vec<String> {
        self.chapter.data.iter().map(|v| format!("{}/{}/{v}", self.base_url, self.chapter.hash)).collect()
    }

    /// Get the full URLs for the data saver images
    pub fn saver_images(&self) -> Vec<String> {
        self.chapter.data_saver.iter().map(|v| format!("{}/{}/{v}", self.base_url, self.chapter.hash)).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AtHomeChapter {
    /// Chapter identifier
    pub hash: String,
    /// Full resolution images
    pub data: Vec<String>,
    /// Lower resolution images
    pub data_saver: Vec<String>,
}
