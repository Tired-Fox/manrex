use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use strum_macros::Display as _;

use crate::client::{request::Param, ExtendParams};

use super::{ContentRating, Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub enum ChapterInclude {
    Manga,
    ScanlationGroup,
    User,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChapterFilter {
    limit: Option<usize>,
    offset: Option<usize>,
    ids: Vec<String>,
    title: Option<String>,
    groups: Vec<String>,
    uploader: Option<Param>,
    manga: Option<String>,
    volumes: Vec<String>,
    chapters: Vec<String>,
    translated_languages: Vec<String>,
    original_languages: Vec<String>,
    excluded_original_languages: Vec<String>,
    content_ratings: Vec<ContentRating>,
    exclude_groups: Vec<String>,
    exclude_uploaders: Vec<String>,
    include_future_updates: Option<bool>,
    include_future_published_at: Option<bool>,
    include_external_url: Option<bool>,
    created_at_since: Option<String>,
    updated_at_since: Option<String>,
    order: BTreeMap<String, Order>,
    includes: Vec<ChapterInclude>,
}
impl ChapterFilter {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn id(mut self, id: impl std::fmt::Display) -> Self {
        self.ids.push(id.to_string());
        self
    }
    pub fn ids<S: std::fmt::Display>(mut self, ids: impl IntoIterator<Item = S>) -> Self {
        self.ids.extend(ids.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn title(mut self, title: impl std::fmt::Display) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn group(mut self, group: impl std::fmt::Display) -> Self {
        self.groups.push(group.to_string());
        self
    }
    pub fn groups<S: std::fmt::Display>(mut self, groups: impl IntoIterator<Item = S>) -> Self {
        self.groups
            .extend(groups.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn uploader(mut self, uploader: impl Into<Param>) -> Self {
        self.uploader = Some(uploader.into());
        self
    }
    pub fn manga(mut self, manga: impl std::fmt::Display) -> Self {
        self.manga = Some(manga.to_string());
        self
    }
    pub fn volume(mut self, volume: impl std::fmt::Display) -> Self {
        self.volumes.push(volume.to_string());
        self
    }
    pub fn volumes<S: std::fmt::Display>(mut self, volumes: impl IntoIterator<Item = S>) -> Self {
        self.volumes
            .extend(volumes.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn chapter(mut self, chapter: impl std::fmt::Display) -> Self {
        self.chapters.push(chapter.to_string());
        self
    }
    pub fn chapters<S: std::fmt::Display>(mut self, chapters: impl IntoIterator<Item = S>) -> Self {
        self.chapters
            .extend(chapters.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn translated_language(mut self, translated_language: impl std::fmt::Display) -> Self {
        self.translated_languages
            .push(translated_language.to_string());
        self
    }
    pub fn translated_languages<S: std::fmt::Display>(
        mut self,
        translated_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.translated_languages
            .extend(translated_languages.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn original_language(mut self, original_language: impl std::fmt::Display) -> Self {
        self.original_languages.push(original_language.to_string());
        self
    }
    pub fn original_languages<S: std::fmt::Display>(
        mut self,
        original_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.original_languages
            .extend(original_languages.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn excluded_original_language(
        mut self,
        excluded_original_language: impl std::fmt::Display,
    ) -> Self {
        self.excluded_original_languages
            .push(excluded_original_language.to_string());
        self
    }
    pub fn excluded_original_languages<S: std::fmt::Display>(
        mut self,
        excluded_original_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.excluded_original_languages.extend(
            excluded_original_languages
                .into_iter()
                .map(|v| v.to_string()),
        );
        self
    }
    pub fn content_rating(mut self, content_rating: ContentRating) -> Self {
        self.content_ratings.push(content_rating);
        self
    }
    pub fn content_ratings(
        mut self,
        content_ratings: impl IntoIterator<Item = ContentRating>,
    ) -> Self {
        self.content_ratings.extend(content_ratings);
        self
    }
    pub fn exclude_group(mut self, exclude_group: impl std::fmt::Display) -> Self {
        self.exclude_groups.push(exclude_group.to_string());
        self
    }
    pub fn exclude_groups<S: std::fmt::Display>(
        mut self,
        exclude_groups: impl IntoIterator<Item = S>,
    ) -> Self {
        self.exclude_groups
            .extend(exclude_groups.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn exclude_uploader(mut self, exclude_uploader: impl std::fmt::Display) -> Self {
        self.exclude_uploaders.push(exclude_uploader.to_string());
        self
    }
    pub fn exclude_uploaders<S: std::fmt::Display>(
        mut self,
        exclude_uploader: impl IntoIterator<Item = S>,
    ) -> Self {
        self.exclude_uploaders
            .extend(exclude_uploader.into_iter().map(|v| v.to_string()));
        self
    }
    pub fn include_future_updates(mut self, include: bool) -> Self {
        self.include_future_updates = Some(include);
        self
    }
    pub fn include_future_published_at(mut self, include: bool) -> Self {
        self.include_future_published_at = Some(include);
        self
    }
    pub fn include_external_url(mut self, include: bool) -> Self {
        self.include_external_url = Some(include);
        self
    }
    pub fn created_at_since(mut self, state: impl std::fmt::Display) -> Self {
        self.created_at_since = Some(state.to_string());
        self
    }
    pub fn updated_at_since(mut self, state: impl std::fmt::Display) -> Self {
        self.updated_at_since = Some(state.to_string());
        self
    }
    pub fn order(mut self, key: impl std::fmt::Display, order: Order) -> Self {
        self.order.insert(key.to_string(), order);
        self
    }
    pub fn orders<S: std::fmt::Display>(
        mut self,
        orders: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order
            .extend(orders.into_iter().map(|(k, v)| (k.to_string(), v)));
        self
    }
    pub fn include(mut self, include: ChapterInclude) -> Self {
        self.includes.push(include);
        self
    }
    pub fn includes(mut self, includes: impl IntoIterator<Item = ChapterInclude>) -> Self {
        self.includes.extend(includes);
        self
    }
}

impl ExtendParams for ChapterFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }
        request.add_param_opt("title", self.title);
        if !self.groups.is_empty() {
            request.add_param("groups", self.groups);
        }
        request.add_param_opt("uploader", self.uploader);
        request.add_param_opt("manga", self.manga);
        if !self.volumes.is_empty() {
            request.add_param("volumes", self.volumes);
        }
        if !self.chapters.is_empty() {
            request.add_param("chapters", self.chapters);
        }
        if !self.translated_languages.is_empty() {
            request.add_param("translatedLanguages", self.translated_languages);
        }
        if !self.original_languages.is_empty() {
            request.add_param("originalLanguages", self.original_languages);
        }
        if !self.excluded_original_languages.is_empty() {
            request.add_param("excludedOriginalLanguages", self.excluded_original_languages);
        }
        if !self.content_ratings.is_empty() {
            request.add_param("contentRatings", self.content_ratings);
        }
        if !self.exclude_groups.is_empty() {
            request.add_param("excludeGroups", self.exclude_groups);
        }
        if !self.exclude_uploaders.is_empty() {
            request.add_param("excludeUploaders", self.exclude_uploaders);
        }
        if let Some(include_future_updates) = self.include_future_updates {
            request.add_param("includeFutureUpdates", if include_future_updates { "1" } else { "0" });
        }
        if let Some(include_future_published_at) = self.include_future_published_at {
            request.add_param("includeFuturePublishedAt", if include_future_published_at { "1" } else { "0" });
        }
        if let Some(include_external_url) = self.include_external_url {
            request.add_param("includeExternalUrl", if include_external_url { "1" } else { "0" });
        }

        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);

        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }

        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    pub title: String,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    /// Count of readable images for this chapter
    pub pages: usize,
    pub translated_language: String,
    pub uploader: Option<String>,
    /// Denotes a chapter that links to an external source
    pub external_url: Option<String>,
    pub version: usize,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub readable_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub id: String,
    pub attributes: ChapterAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChapter {
    pub version: usize,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub chapter: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub translated_language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<String>>,
}

impl UpdateChapter {
    pub fn version(mut self, version: usize) -> Self {
        self.version = version;
        self
    }

    pub fn title(mut self, title: impl std::fmt::Display) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn volume(mut self, volume: impl std::fmt::Display) -> Self {
        self.volume = Some(volume.to_string());
        self
    }

    pub fn chapter(mut self, chapter: impl std::fmt::Display) -> Self {
        self.chapter = Some(chapter.to_string());
        self
    }

    pub fn translated_language(mut self, translated_language: impl std::fmt::Display) -> Self {
        self.translated_language = Some(translated_language.to_string());
        self
    }

    pub fn groups<S: std::fmt::Display>(mut self, groups: impl IntoIterator<Item=S>) -> Self {
        self.groups = Some(groups.into_iter().map(|v| v.to_string()).collect());
        self
    }
}

impl Default for UpdateChapter {
    fn default() -> Self {
        Self {
            version: 1,
            title: None,
            volume: None,
            chapter: None,
            translated_language: None,
            groups: None,
        }
    }
}
