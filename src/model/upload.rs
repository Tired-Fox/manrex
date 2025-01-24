use serde::{Deserialize, Serialize};

use super::IntoData;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSessionAttributes {
    pub version: usize,
    pub created_at: String,
    pub updated_at: String,

    #[serde(rename="isCommitted")]
    pub committed: bool,
    #[serde(rename="isProcessed")]
    pub processed: bool,
    #[serde(rename="isDeleted")]
    pub deleted: bool,
}

impl UploadSessionAttributes {
    pub fn is_committed(&self) -> bool {
        self.committed
    }
    pub fn is_processed(&self) -> bool {
        self.processed
    }
    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSession {
    pub id: String,
    pub attributes: UploadSessionAttributes,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display)]
#[serde(rename_all="snake_case")]
#[strum(serialize_all="snake_case")]
pub enum FileSource {
    Local,
    Remote,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadSessionAttributes {
    pub original_file_name: String,
    pub file_hash: String,
    pub file_size: usize,
    pub mime_type: String,
    pub source: FileSource,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadSession {
    pub id: String,
    pub attributes: FileUploadSessionAttributes,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterDraft {
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub title: Option<String>,
    pub translated_language: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub publish_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiresApproval {
    pub requires_approval: bool
}

impl IntoData<bool> for RequiresApproval {
    fn into_data(self) -> bool {
        self.requires_approval
    }
}
