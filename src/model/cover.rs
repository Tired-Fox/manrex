use std::{borrow::Cow, collections::BTreeMap, path::Path};

use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::{
    client::{ExtendParams, Optional},
    uuid::{CoverId, MangaId, UserId},
    Error,
};

use super::{Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CoverInclude {
    Manga,
    User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverArtFilter {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub manga: Vec<MangaId>,
    pub ids: Vec<CoverId>,
    pub uploaders: Vec<UserId>,
    pub locales: Vec<String>,
    pub order: BTreeMap<String, Order>,
    pub includes: Vec<CoverInclude>,
}

impl CoverArtFilter {
    pub fn limit(mut self, s: usize) -> Self {
        self.limit = Some(s);
        self
    }

    pub fn offset(mut self, s: usize) -> Self {
        self.offset = Some(s);
        self
    }

    pub fn manga<M: Into<MangaId>>(mut self, s: impl IntoIterator<Item = M>) -> Self {
        self.manga = s.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn ids<C: Into<CoverId>>(mut self, s: impl IntoIterator<Item = C>) -> Self {
        self.ids = s.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn uploaders<U: Into<UserId>>(mut self, s: impl IntoIterator<Item = U>) -> Self {
        self.uploaders = s.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn locales<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item = S>) -> Self {
        self.locales.extend(s.into_iter().map(|v| v.to_string()));
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item = (S, Order)>) -> Self {
        self.order = s.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self
    }

    pub fn includes(mut self, s: impl IntoIterator<Item = CoverInclude>) -> Self {
        self.includes.extend(s);
        self
    }
}

impl ExtendParams for CoverArtFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        if !self.manga.is_empty() {
            request.add_param("manga", self.manga);
        }
        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }
        if !self.uploaders.is_empty() {
            request.add_param("uploaders", self.uploaders);
        }
        if !self.locales.is_empty() {
            request.add_param("locales", self.locales);
        }
        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }
        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub volume: Option<String>,
    pub file_name: String,
    pub description: Option<String>,
    pub locale: Option<String>,
    pub version: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    pub id: CoverId,
    pub attributes: CoverAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CoverSize {
    /// 512 pixels wide
    #[strum(to_string = "512")]
    Large,
    /// 256 pixels wide
    #[strum(to_string = "256")]
    Small,
}

#[derive(Debug)]
pub struct UploadCover(multipart::Form);
impl From<UploadCover> for multipart::Form {
    fn from(value: UploadCover) -> Self {
        value.0
    }
}

impl UploadCover {
    pub async fn new(file: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self(multipart::Form::new().file("file", file).await?))
    }

    pub fn volume(mut self, volume: impl Into<Cow<'static, str>>) -> Self {
        self.0 = self.0.text("volume", volume);
        self
    }

    pub fn description(mut self, description: impl Into<Cow<'static, str>>) -> Self {
        self.0 = self.0.text("description", description);
        self
    }

    pub fn locale(mut self, locale: impl Into<Cow<'static, str>>) -> Self {
        self.0 = self.0.text("locale", locale);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditCover {
    pub volume: Option<String>,
    pub version: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

impl Default for EditCover {
    fn default() -> Self {
        Self {
            volume: None,
            version: 1,
            description: None,
            locale: None,
        }
    }
}

impl EditCover {
    pub fn new(volume: impl Optional<String>, version: usize) -> Self {
        Self {
            volume: volume.optional(),
            version,
            ..Default::default()
        }
    }

    pub fn description(mut self, description: impl std::fmt::Display) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn locale(mut self, locale: impl std::fmt::Display) -> Self {
        self.locale = Some(locale.to_string());
        self
    }
}
