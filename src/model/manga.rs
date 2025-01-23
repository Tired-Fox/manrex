use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::client::ExtendParams;

use super::{ContentRating, Demographic, MangaState, Relationship, Status, TagGroup};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub enum MangaInclude {
    Manga,
    Author,
    CoverArt,
    Artist,
    Tag,
    Creator,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaFilter {
    title: Option<String>,
    includes: Vec<MangaInclude>,
}

impl MangaFilter {
    pub fn title(mut self, s: impl std::fmt::Display) -> Self {
        self.title = Some(s.to_string());
        self
    }

    pub fn includes(mut self, s: impl IntoIterator<Item=MangaInclude>) -> Self {
        self.includes = s.into_iter().collect();
        self
    }
}

impl ExtendParams for MangaFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("title", self.title);
        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    name: BTreeMap<String, String>,
    description: BTreeMap<String, String>,
    group: TagGroup,
    version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    id: String,
    attributes: TagAttributes,
    relationships: Vec<Relationship>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    title: BTreeMap<String, String>,
    alt_titles: Vec<BTreeMap<String, String>>,
    description: BTreeMap<String, String>,
    is_locked: bool,
    links: BTreeMap<String, String>,
    original_language: String,
    last_volume: Option<String>,
    last_chapter: Option<String>,
    publication_demographic: Option<Demographic>,
    status: Option<Status>,
    year: Option<usize>,
    content_rating: ContentRating,
    chapter_numbers_reset_on_new_volume: bool,
    available_translated_languages: Vec<String>,
    latest_uploaded_chapter: String,
    tags: Vec<Tag>,
    state: MangaState,
    version: usize,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    pub id: String,
    pub attributes: MangaAttributes,
    pub relationships: Vec<Relationship>
}
