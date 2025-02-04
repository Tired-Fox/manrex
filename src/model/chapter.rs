use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    client::{request::OneOrMany, ExtendParams},
    uuid::{ChapterId, GroupId, MangaId, UserId},
};

use super::{ContentRating, Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChapterInclude {
    Manga,
    ScanlationGroup,
    User,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChapterFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<ChapterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader: Option<OneOrMany<UserId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga: Option<MangaId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<OneOrMany<ChapterId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_language: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_original_language: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<Vec<ContentRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uploaders: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_published_at: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<ChapterInclude>>,
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
    pub fn ids<C: Into<ChapterId>>(mut self, ids: impl IntoIterator<Item = C>) -> Self {
        self.ids = Some(ids.into_iter().map(|v| v.into()).collect());
        self
    }
    pub fn title(mut self, title: impl std::fmt::Display) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub fn group<G: Into<GroupId>>(mut self, groups: impl IntoIterator<Item = G>) -> Self {
        self.groups = Some(groups.into_iter().map(|v| v.into()).collect());
        self
    }
    pub fn uploader(mut self, uploader: impl Into<OneOrMany<UserId>>) -> Self {
        self.uploader = Some(uploader.into());
        self
    }
    pub fn manga(mut self, manga: impl Into<MangaId>) -> Self {
        self.manga = Some(manga.into());
        self
    }
    pub fn volumes<S: std::fmt::Display>(mut self, volumes: impl IntoIterator<Item = S>) -> Self {
        self.volume = Some(volumes.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn chapters(mut self, chapters: impl Into<OneOrMany<ChapterId>>) -> Self {
        self.chapter = Some(chapters.into());
        self
    }
    pub fn translated_languages<S: std::fmt::Display>(
        mut self,
        translated_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.translated_language = Some(translated_languages.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn original_languages<S: std::fmt::Display>(
        mut self,
        original_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.original_language = Some(original_languages.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn excluded_original_languages<S: std::fmt::Display>(
        mut self,
        excluded_original_languages: impl IntoIterator<Item = S>,
    ) -> Self {
        self.excluded_original_language = Some(
            excluded_original_languages
                .into_iter()
                .map(|v| v.to_string()).collect()
        );
        self
    }
    pub fn content_ratings(
        mut self,
        content_ratings: impl IntoIterator<Item = ContentRating>,
    ) -> Self {
        self.content_rating = Some(content_ratings.into_iter().collect());
        self
    }
    pub fn exclude_groups<S: std::fmt::Display>(
        mut self,
        exclude_groups: impl IntoIterator<Item = S>,
    ) -> Self {
        self.exclude_groups = Some(exclude_groups.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn exclude_uploaders<S: std::fmt::Display>(
        mut self,
        exclude_uploader: impl IntoIterator<Item = S>,
    ) -> Self {
        self.exclude_uploaders = Some(exclude_uploader.into_iter().map(|v| v.to_string()).collect());
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
    pub fn orders<S: std::fmt::Display>(
        mut self,
        orders: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order = Some(orders.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        self
    }
    pub fn includes(mut self, includes: impl IntoIterator<Item = ChapterInclude>) -> Self {
        self.includes = Some(includes.into_iter().collect());
        self
    }
}

impl ExtendParams for ChapterFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("ids", self.ids);
        request.add_param_opt("title", self.title);
        request.add_param_opt("groups", self.groups);
        request.add_param_opt("uploader", self.uploader);
        request.add_param_opt("manga", self.manga);
        request.add_param_opt("volume", self.volume);
        request.add_param_opt("chapter", self.chapter);
        request.add_param_opt("translatedLanguage", self.translated_language);
        request.add_param_opt("originalLanguage", self.original_language);
        request.add_param_opt(
            "excludedOriginalLanguage",
            self.excluded_original_language,
        );
        request.add_param_opt("contentRating", self.content_rating);
        request.add_param_opt("excludeGroups", self.exclude_groups);
        request.add_param_opt("excludeUploaders", self.exclude_uploaders);
        if let Some(include_future_updates) = self.include_future_updates {
            request.add_param(
                "includeFutureUpdates",
                if include_future_updates { "1" } else { "0" },
            );
        }
        if let Some(include_future_published_at) = self.include_future_published_at {
            request.add_param(
                "includeFuturePublishedAt",
                if include_future_published_at {
                    "1"
                } else {
                    "0"
                },
            );
        }
        if let Some(include_external_url) = self.include_external_url {
            request.add_param(
                "includeExternalUrl",
                if include_external_url { "1" } else { "0" },
            );
        }

        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);
        request.add_param_opt("order", self.order);
        request.add_param_opt("includes", self.includes);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    /// Denotes a chapter that links to an external source
    pub external_url: Option<String>,
    /// Count of readable images for this chapter
    pub pages: usize,
    pub version: usize,

    pub translated_language: Option<String>,
    pub uploader: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
    pub readable_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub id: ChapterId,
    pub attributes: ChapterAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChapter {
    pub version: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn groups<S: std::fmt::Display>(mut self, groups: impl IntoIterator<Item = S>) -> Self {
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
