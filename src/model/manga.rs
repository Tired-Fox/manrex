use std::{collections::{BTreeMap, HashSet}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{client::{ExtendParams, MangaDex, Optional}, Error};

use super::{chapter::ChapterInclude, cover::CoverSize, ContentRating, Demographic, Image, IntoData, MangaState, Order, Relation, Relationship, RelationshipAttributes, Status, TagGroup, TagMode};

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
    ids: Vec<String>,
    includes: Vec<MangaInclude>,

    limit: Option<usize>,
    offset: Option<usize>,

    author_or_artist: Option<String>,
    authors: Vec<String>,
    artists: Vec<String>,

    year: Option<String>,

    included_tags: Vec<String>,
    included_tags_mode: Option<TagMode>,

    excluded_tags: Vec<String>,
    excluded_tags_mode: Option<TagMode>,

    status: Vec<Status>,

    original_languages: Vec<String>,
    excluded_original_languages: Vec<String>,

    available_translated_languages: Vec<String>,

    publication_demographic: HashSet<Demographic>,

    content_ratings: HashSet<ContentRating>,

    created_at_since: Option<String>,
    updated_at_since: Option<String>,

    order: BTreeMap<String, Order>,

    has_available_chapters: Option<bool>,
    group: Option<String>,
}

impl MangaFilter {
    pub fn title(mut self, s: impl std::fmt::Display) -> Self {
        self.title = Some(s.to_string());
        self
    }

    pub fn ids<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.ids = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn includes(mut self, s: impl IntoIterator<Item=MangaInclude>) -> Self {
        self.includes = s.into_iter().collect();
        self
    }

    pub fn limit(mut self, s: usize) -> Self {
        self.limit = Some(s);
        self
    }

    pub fn offset(mut self, s: usize) -> Self {
        self.offset = Some(s);
        self
    }

    pub fn author_or_artist(mut self, s: impl std::fmt::Display) -> Self {
        self.author_or_artist = Some(s.to_string());
        self
    }

    pub fn authors<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.authors = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn artists<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.artists = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn year(mut self, s: impl std::fmt::Display) -> Self {
        self.year = Some(s.to_string());
        self
    }

    pub fn included_tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.included_tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn included_tags_mode(mut self, s: TagMode) -> Self {
        self.included_tags_mode = Some(s);
        self
    }

    pub fn excluded_tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn excluded_tags_mode(mut self, s: TagMode) -> Self {
        self.excluded_tags_mode = Some(s);
        self
    }

    pub fn status(mut self, s: impl IntoIterator<Item=Status>) -> Self {
        self.status = s.into_iter().collect();
        self
    }

    pub fn original_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.original_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn excluded_original_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_original_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn available_translated_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.available_translated_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn publication_demographic(mut self, s: impl IntoIterator<Item=Demographic>) -> Self {
        self.publication_demographic = s.into_iter().collect();
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item=ContentRating>) -> Self {
        self.content_ratings = s.into_iter().collect();
        self
    }

    pub fn created_at_since(mut self, s: impl std::fmt::Display) -> Self {
        self.created_at_since = Some(s.to_string());
        self
    }
    pub fn updated_at_since(mut self, s: impl std::fmt::Display) -> Self {
        self.updated_at_since = Some(s.to_string());
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.order = s.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self
    }

    pub fn has_available_chapters(mut self, s: bool) -> Self {
        self.has_available_chapters = Some(s);
        self
    }

    pub fn group(mut self, s: impl std::fmt::Display) -> Self {
        self.group = Some(s.to_string());
        self
    }
}

impl ExtendParams for MangaFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("title", self.title);

        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }

        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }

        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);

        request.add_param_opt("authorOrArtist", self.author_or_artist);
        if !self.authors.is_empty() {
            request.add_param("authors", self.authors);
        }
        if !self.artists.is_empty() {
            request.add_param("artists", self.artists);
        }

        request.add_param_opt("year", self.year);

        if !self.included_tags.is_empty() {
            request.add_param("includedTags", self.included_tags);
        }
        request.add_param_opt("includedTagsMode", self.included_tags_mode.map(|v| v.to_string()));

        if !self.excluded_tags.is_empty() {
            request.add_param("excludedTags", self.excluded_tags);
        }
        request.add_param_opt("excludedTagsMode", self.excluded_tags_mode.map(|v| v.to_string()));

        if !self.status.is_empty() {
            request.add_param("status", self.status);
        }

        if !self.original_languages.is_empty() {
            request.add_param("originalLanguages", self.original_languages);
        }
        if !self.excluded_original_languages.is_empty() {
            request.add_param("excludedOriginalLanguages", self.excluded_original_languages);
        }

        if !self.available_translated_languages.is_empty() {
            request.add_param("availableTranslatedLanguages", self.available_translated_languages);
        }

        if !self.publication_demographic.is_empty() {
            request.add_param("publicationDemographic", self.publication_demographic);
        }

        if !self.content_ratings.is_empty() {
            request.add_param("contentRatings", self.content_ratings);
        }

        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);

        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }

        request.add_param_opt("hasAvailableChapters", self.has_available_chapters);
        request.add_param_opt("group", self.group);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    pub name: BTreeMap<String, String>,
    pub description: BTreeMap<String, String>,
    pub group: TagGroup,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub attributes: TagAttributes,
    pub relationships: Vec<Relationship>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: BTreeMap<String, String>,
    pub alt_titles: Vec<BTreeMap<String, String>>,
    pub description: BTreeMap<String, String>,
    pub is_locked: bool,
    pub links: BTreeMap<String, String>,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<Demographic>,
    pub status: Option<Status>,
    pub year: Option<usize>,
    pub content_rating: ContentRating,
    pub chapter_numbers_reset_on_new_volume: bool,
    pub available_translated_languages: Vec<String>,
    pub latest_uploaded_chapter: String,
    pub tags: Vec<Tag>,
    pub state: MangaState,
    pub version: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    pub id: String,
    pub attributes: MangaAttributes,
    pub relationships: Vec<Relationship>
}

impl Manga {
    pub fn get_cover_art<M>(
        &self,
        size: impl Optional<CoverSize, M>
    ) -> Result<Image, Error> {
        let manga_id = self.id.as_str();

        let cover_art = self.relationships
            .iter()
            .filter_map(|r| {
                match r.attributes.as_ref() {
                    Some(RelationshipAttributes::CoverArt(cover)) => Some(cover),
                    _ => None
                }
            })
            .collect::<Vec<_>>();

        let cover_art = cover_art
            .first()
            .ok_or(Error::custom("missing cover art relationship attributes. Make sure to add `CoverArt` to the filter includes when fetching the manga"))?;

        let file_name = cover_art.file_name.as_str();

        let file_name = if let Some(size) = size.optional() {
            format!("{file_name}.{size}.jpg")
        } else {
            file_name.to_string()
        };

        Ok(Image {
            url: format!("{}/covers/{manga_id}/{file_name}", MangaDex::Uploads),
            expires: None,
            file_name: Some(PathBuf::from(file_name))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Volumes<D> {
    pub volumes: D
}
impl<D> IntoData<D> for Volumes<D> {
    fn into_data(self) -> D {
        self.volumes
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Volume {
    pub volume: String,
    pub count: usize,
    pub chapters: BTreeMap<String, VolumeChapter>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct VolumeChapter {
    pub chapter: String,
    pub id: String,
    pub others: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CreateManga {
    title: String,
    original_language: String,
    status: Status,
    content_rating: ContentRating,

    #[serde(skip_serializing_if="BTreeMap::is_empty")]
    alt_titles: BTreeMap<String, String>,
    #[serde(skip_serializing_if="BTreeMap::is_empty")]
    description: BTreeMap<String, String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    authors: Vec<String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    artists: Vec<String>,
    #[serde(skip_serializing_if="BTreeMap::is_empty")]
    links: BTreeMap<String, String>,

    #[serde(skip_serializing_if="Option::is_none")]
    last_volume: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    last_chapter: Option<String>,

    #[serde(skip_serializing_if="HashSet::is_empty")]
    publication_demographic: HashSet<Demographic>,
    #[serde(skip_serializing_if="Option::is_none")]
    year: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    primary_cover: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    version: Option<usize>
}

impl CreateManga {
    pub fn new(
        title: impl std::fmt::Display,
        original_language: impl std::fmt::Display,
        status: Status,
        content_rating: ContentRating,
    ) -> Self {
        Self {
            title: title.to_string(),
            original_language: original_language.to_string(),
            status,
            content_rating,

            alt_titles: Default::default(),
            description: Default::default(),
            authors: Default::default(),
            artists: Default::default(),
            links: Default::default(),
            last_volume: Default::default(),
            last_chapter: Default::default(),
            publication_demographic: Default::default(),
            year: Default::default(),
            chapter_numbers_reset_on_new_volume: Default::default(),
            tags: Default::default(),
            primary_cover: Default::default(),
            version: Default::default(),
        }
    }

    pub fn last_volume(mut self, s: impl std::fmt::Display) -> Self {
        self.last_volume = Some(s.to_string());
        self
    }
    pub fn last_chapter(mut self, s: impl std::fmt::Display) -> Self {
        self.last_chapter = Some(s.to_string());
        self
    }
    pub fn primary_cover(mut self, s: impl std::fmt::Display) -> Self {
        self.primary_cover = Some(s.to_string());
        self
    }
    pub fn year(mut self, s: usize) -> Self {
        self.year = Some(s);
        self
    }
    pub fn chapter_numbers_reset_on_new_volume(mut self, s: bool) -> Self {
        self.chapter_numbers_reset_on_new_volume = Some(s);
        self
    }
    pub fn version(mut self, s: usize) -> Self {
        self.version = Some(s);
        self
    }
    pub fn alt_titles<S1: std::fmt::Display, S2: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S1, S2)>) -> Self {
        self.alt_titles = s.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }
    pub fn description<S1: std::fmt::Display, S2: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S1, S2)>) -> Self {
        self.description = s.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }
    pub fn links<S1: std::fmt::Display, S2: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S1, S2)>) -> Self {
        self.links = s.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }
    pub fn authors<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.authors = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn artists<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.artists = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn publication_demographic(mut self, s: impl IntoIterator<Item=Demographic>) -> Self {
        self.publication_demographic = s.into_iter().collect();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct UpdateManga {
    version: usize,

    #[serde(skip_serializing_if="Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    original_language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    status: Option<Status>,
    #[serde(skip_serializing_if="Option::is_none")]
    content_rating: Option<ContentRating>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    alt_titles: Vec<BTreeMap<String, String>>,
    #[serde(skip_serializing_if="BTreeMap::is_empty")]
    description: BTreeMap<String, String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    authors: Vec<String>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    artists: Vec<String>,
    #[serde(skip_serializing_if="BTreeMap::is_empty")]
    links: BTreeMap<String, String>,

    #[serde(skip_serializing_if="Option::is_none")]
    last_volume: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    last_chapter: Option<String>,

    #[serde(skip_serializing_if="HashSet::is_empty")]
    publication_demographic: HashSet<Demographic>,
    #[serde(skip_serializing_if="Option::is_none")]
    year: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")]
    chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(skip_serializing_if="Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    primary_cover: Option<String>,
}

impl UpdateManga {
    pub fn new(
        version: usize,
    ) -> Self {
        Self {
            version,

            title: Default::default(),
            alt_titles: Default::default(),
            description: Default::default(),
            authors: Default::default(),
            artists: Default::default(),
            links: Default::default(),
            original_language: Default::default(),
            last_volume: Default::default(),
            last_chapter: Default::default(),
            publication_demographic: Default::default(),
            status: Default::default(),
            year: Default::default(),
            content_rating: Default::default(),
            chapter_numbers_reset_on_new_volume: Default::default(),
            tags: Default::default(),
            primary_cover: Default::default(),
        }
    }

    pub fn title(mut self, s: impl std::fmt::Display) -> Self {
        self.title = Some(s.to_string());
        self
    }
    pub fn original_language(mut self, s: impl std::fmt::Display) -> Self {
        self.original_language = Some(s.to_string());
        self
    }
    pub fn status(mut self, s: Status) -> Self {
        self.status = Some(s);
        self
    }
    pub fn content_rating(mut self, s: ContentRating) -> Self {
        self.content_rating = Some(s);
        self
    }

    pub fn last_volume(mut self, s: impl std::fmt::Display) -> Self {
        self.last_volume = Some(s.to_string());
        self
    }
    pub fn last_chapter(mut self, s: impl std::fmt::Display) -> Self {
        self.last_chapter = Some(s.to_string());
        self
    }
    pub fn primary_cover(mut self, s: impl std::fmt::Display) -> Self {
        self.primary_cover = Some(s.to_string());
        self
    }
    pub fn year(mut self, s: usize) -> Self {
        self.year = Some(s);
        self
    }
    pub fn chapter_numbers_reset_on_new_volume(mut self, s: bool) -> Self {
        self.chapter_numbers_reset_on_new_volume = Some(s);
        self
    }
    pub fn alt_titles<S1: std::fmt::Display, S2: std::fmt::Display, I: IntoIterator<Item=(S1, S2)>>(mut self, s: impl IntoIterator<Item=I>) -> Self {
        self.alt_titles = s.into_iter().map(|i| i.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()).collect();
        self
    }
    pub fn description<S1: std::fmt::Display, S2: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S1, S2)>) -> Self {
        self.description = s.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }
    pub fn links<S1: std::fmt::Display, S2: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S1, S2)>) -> Self {
        self.links = s.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        self
    }
    pub fn authors<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.authors = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn artists<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.artists = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn publication_demographic(mut self, s: impl IntoIterator<Item=Demographic>) -> Self {
        self.publication_demographic = s.into_iter().collect();
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FeedFilter {
    limit: Option<usize>,
    offset: Option<usize>,

    translated_languages: Vec<String>,
    original_languages: Vec<String>,
    excluded_original_languages: Vec<String>,
    content_ratings: HashSet<ContentRating>,
    excluded_groups: Vec<String>,
    excluded_uploaders: Vec<String>,

    created_at_since: Option<String>,
    updated_at_since: Option<String>,
    publish_at_since: Option<String>,
    order: BTreeMap<String, Order>,
    includes: Vec<ChapterInclude>,

    include_future_updates: Option<bool>,
    include_empty_pages: Option<bool>,
    include_future_publish_at: Option<bool>,
    include_external_url: Option<bool>,
}

impl FeedFilter {
    pub fn limit(mut self, s: usize) -> Self {
        self.limit = Some(s);
        self
    }
    pub fn offset(mut self, s: usize) -> Self {
        self.offset = Some(s);
        self
    }
    pub fn include_future_updates(mut self, s: bool) -> Self {
        self.include_future_updates = Some(s);
        self
    }
    pub fn include_empty_pages(mut self, s: bool) -> Self {
        self.include_empty_pages = Some(s);
        self
    }
    pub fn include_future_publish_at(mut self, s: bool) -> Self {
        self.include_future_publish_at = Some(s);
        self
    }
    pub fn include_external_url(mut self, s: bool) -> Self {
        self.include_external_url = Some(s);
        self
    }

    pub fn translated_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.translated_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn original_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.original_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn excluded_original_languages<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_original_languages = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn excluded_groups<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_groups = s.into_iter().map(|v| v.to_string()).collect();
        self
    }
    pub fn excluded_uploaders<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_uploaders = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item=ContentRating>) -> Self {
        self.content_ratings = s.into_iter().collect();
        self
    }
    pub fn includes(mut self, s: impl IntoIterator<Item=ChapterInclude>) -> Self {
        self.includes = s.into_iter().collect();
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.order = s.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self
    }

    pub fn created_at_since(mut self, s: impl std::fmt::Display) -> Self {
        self.created_at_since = Some(s.to_string());
        self
    }
    pub fn updated_at_since(mut self, s: impl std::fmt::Display) -> Self {
        self.updated_at_since = Some(s.to_string());
        self
    }
    pub fn publish_at_since(mut self, s: impl std::fmt::Display) -> Self {
        self.publish_at_since = Some(s.to_string());
        self
    }
}

impl ExtendParams for FeedFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);

        if !self.translated_languages.is_empty() {
            request.add_param("translatedLanguage", self.translated_languages);
        }
        if !self.original_languages.is_empty() {
            request.add_param("originalLanguage", self.original_languages);
        }
        if !self.excluded_original_languages.is_empty() {
            request.add_param("excludedOriginalLanguage", self.excluded_original_languages);
        }
        if !self.content_ratings.is_empty() {
            request.add_param("contentRatings", self.content_ratings);
        }
        if !self.excluded_groups.is_empty() {
            request.add_param("excludedGroups", self.excluded_groups);
        }
        if !self.excluded_uploaders.is_empty() {
            request.add_param("excludedUploaders", self.excluded_uploaders);
        }

        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);
        request.add_param_opt("publishAtSince", self.publish_at_since);
        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }
        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }

        if let Some(s) = self.include_future_updates {
            request.add_param("includeFutureUpdates", if s { "1" } else { "0" });
        }
        if let Some(s) = self.include_empty_pages {
            request.add_param("includeEmptyPages", if s { "1" } else { "0" });
        }
        if let Some(s) = self.include_future_publish_at {
            request.add_param("includeFuturePublishAt", if s { "1" } else { "0" });
        }
        if let Some(s) = self.include_external_url {
            request.add_param("includeExternalUrl", if s { "1" } else { "0" });
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RandomMangaFilter {
    includes: Vec<MangaInclude>,
    content_ratings: HashSet<ContentRating>,
    included_tags: Vec<String>,
    included_tags_mode: Option<TagMode>,
    excluded_tags: Vec<String>,
    excluded_tags_mode: Option<TagMode>,
}

impl RandomMangaFilter {
    pub fn includes(mut self, s: impl IntoIterator<Item=MangaInclude>) -> Self {
        self.includes = s.into_iter().collect();
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item=ContentRating>) -> Self {
        self.content_ratings = s.into_iter().collect();
        self
    }

    pub fn included_tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.included_tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn included_tags_mode(mut self, s: TagMode) -> Self {
        self.included_tags_mode = Some(s);
        self
    }

    pub fn excluded_tags<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=S>) -> Self {
        self.excluded_tags = s.into_iter().map(|v| v.to_string()).collect();
        self
    }

    pub fn excluded_tags_mode(mut self, s: TagMode) -> Self {
        self.excluded_tags_mode = Some(s);
        self
    }
}

impl ExtendParams for RandomMangaFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }

        if !self.content_ratings.is_empty() {
            request.add_param("contentRating", self.content_ratings);
        }

        if !self.included_tags.is_empty() {
            request.add_param("includedTags", self.included_tags);
        }
        request.add_param_opt("includedTagsMode", self.included_tags_mode.map(|v| v.to_string()));

        if !self.excluded_tags.is_empty() {
            request.add_param("excludedTags", self.excluded_tags);
        }
        request.add_param_opt("excludedTagsMode", self.excluded_tags_mode.map(|v| v.to_string()));
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Statuses<D> {
    statuses: D
}
impl<D> IntoData<D> for Statuses<D> {
    fn into_data(self) -> D {
        self.statuses
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DataStatus {
    status: Status
}
impl IntoData<Status> for DataStatus {
    fn into_data(self) -> Status {
        self.status
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DraftFilter {
    limit: Option<usize>,
    offset: Option<usize>,
    state: Option<MangaState>,
    order: BTreeMap<String, Order>,
    includes: Vec<MangaInclude>,
}

impl DraftFilter {
    pub fn limit(mut self, s: usize) -> Self {
        self.limit = Some(s);
        self
    }

    pub fn offset(mut self, s: usize) -> Self {
        self.offset = Some(s);
        self
    }

    pub fn state(mut self, s: MangaState) -> Self {
        self.state = Some(s);
        self
    }

    pub fn includes(mut self, s: impl IntoIterator<Item=MangaInclude>) -> Self {
        self.includes = s.into_iter().collect();
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.order = s.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        self
    }
}

impl ExtendParams for DraftFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }

        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("state", self.state.map(|v| v.to_string()));
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MangaRelationAttributes {
    relation: Relation,
    version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MangaRelation {
    id: String,
    attributes: MangaRelationAttributes,
    relationships: Vec<Relationship>
}
