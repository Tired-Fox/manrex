use std::{
    collections::{BTreeMap, HashSet},
};

use serde::{Deserialize, Serialize};

use crate::{
    client::{ExtendParams, MangaDex, Optional},
    uuid::{ArtistId, AuthorId, ChapterId, CoverId, GroupId, MangaId, TagId, UserId},
    Error, Uuid,
};

use super::{
    chapter::ChapterInclude, cover::CoverSize, ContentRating, Demographic, Image, IntoData,
    MangaState, Order, Relation, Relationship, RelationshipAttributes, Status, TagGroup, TagMode,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MangaInclude {
    Manga,
    Author,
    CoverArt,
    Artist,
    Tag,
    Creator,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<MangaId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<MangaInclude>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_or_artist: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_tags: Option<Vec<TagId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_tags_mode: Option<TagMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags: Option<Vec<TagId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags_mode: Option<TagMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<Status>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_original_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_translated_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_demographic: Option<HashSet<Demographic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_ratings: Option<HashSet<ContentRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_available_chapters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl MangaFilter {
    pub fn title(mut self, s: impl std::fmt::Display) -> Self {
        self.title = Some(s.to_string());
        self
    }

    pub fn ids<M: Into<MangaId>>(mut self, s: impl IntoIterator<Item = M>) -> Self {
        self.ids = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn includes(mut self, s: impl IntoIterator<Item = MangaInclude>) -> Self {
        self.includes = Some(s.into_iter().collect());
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

    pub fn author_or_artist(mut self, s: impl Into<Uuid>) -> Self {
        self.author_or_artist = Some(s.into());
        self
    }

    pub fn authors<A: Into<AuthorId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.authors = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn artists<A: Into<ArtistId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.artists = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn year(mut self, s: impl std::fmt::Display) -> Self {
        self.year = Some(s.to_string());
        self
    }

    pub fn included_tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.included_tags = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn included_tags_mode(mut self, s: TagMode) -> Self {
        self.included_tags_mode = Some(s);
        self
    }

    pub fn excluded_tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.excluded_tags = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn excluded_tags_mode(mut self, s: TagMode) -> Self {
        self.excluded_tags_mode = Some(s);
        self
    }

    pub fn status(mut self, s: impl IntoIterator<Item = Status>) -> Self {
        self.status = Some(s.into_iter().collect());
        self
    }

    pub fn original_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.original_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }

    pub fn excluded_original_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.excluded_original_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }

    pub fn available_translated_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.available_translated_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }

    pub fn publication_demographic(mut self, s: impl IntoIterator<Item = Demographic>) -> Self {
        self.publication_demographic = Some(s.into_iter().collect());
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item = ContentRating>) -> Self {
        self.content_ratings = Some(s.into_iter().collect());
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

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item = (S, Order)>) -> Self {
        self.order = Some(s.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
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
        request.add_param_opt("includes", self.includes);
        request.add_param_opt("ids", self.ids);
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("authorOrArtist", self.author_or_artist);
        request.add_param_opt("authors", self.authors);
        request.add_param_opt("artists", self.artists);
        request.add_param_opt("year", self.year);
        request.add_param_opt("includedTags", self.included_tags);
        request.add_param_opt(
            "includedTagsMode",
            self.included_tags_mode.map(|v| v.to_string()),
        );
        request.add_param_opt("excludedTags", self.excluded_tags);
        request.add_param_opt(
            "excludedTagsMode",
            self.excluded_tags_mode.map(|v| v.to_string()),
        );
        request.add_param_opt("status", self.status);
        request.add_param_opt("originalLanguages", self.original_languages);
        request.add_param_opt(
            "excludedOriginalLanguages",
            self.excluded_original_languages,
        );
        request.add_param_opt(
            "availableTranslatedLanguages",
            self.available_translated_languages,
        );
        request.add_param_opt("publicationDemographic", self.publication_demographic);
        request.add_param_opt("contentRatings", self.content_ratings);
        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);
        request.add_param_opt("order", self.order);
        request.add_param_opt("hasAvailableChapters", self.has_available_chapters);
        request.add_param_opt("group", self.group);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAttributes {
    pub name: BTreeMap<String, String>,
    pub description: BTreeMap<String, String>,
    pub group: TagGroup,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: TagId,
    pub attributes: TagAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    pub id: MangaId,
    pub attributes: MangaAttributes,
    pub relationships: Vec<Relationship>,
}

impl Manga {
    pub fn get_cover_art<M>(&self, size: impl Optional<CoverSize, M>) -> Result<Image, Error> {
        let manga_id = self.id.as_ref();

        let cover_art = self
            .relationships
            .iter()
            .filter_map(|r| match r.attributes.as_ref() {
                Some(RelationshipAttributes::CoverArt(cover)) => Some(cover),
                _ => None,
            })
            .collect::<Vec<_>>();

        let cover_art = cover_art
            .first()
            .ok_or(Error::custom("missing cover art relationship attributes. Make sure to add `CoverArt` to the filter includes when fetching the manga"))?
            .as_ref()
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
            file_name: file_name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volumes<D> {
    pub volumes: D,
}
impl<D> IntoData<D> for Volumes<D> {
    fn into_data(self) -> D {
        self.volumes
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub volume: String,
    pub count: usize,
    pub chapters: BTreeMap<String, VolumeChapter>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeChapter {
    pub chapter: String,
    pub id: ChapterId,
    pub others: Vec<String>,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateManga {
    pub title: String,
    pub original_language: String,
    pub status: Status,
    pub content_rating: ContentRating,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub alt_titles: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<AuthorId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artists: Vec<ArtistId>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chapter: Option<ChapterId>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub publication_demographic: HashSet<Demographic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cover: Option<CoverId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<usize>,
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
    pub fn last_chapter(mut self, s: impl Into<ChapterId>) -> Self {
        self.last_chapter = Some(s.into());
        self
    }
    pub fn primary_cover(mut self, s: impl Into<CoverId>) -> Self {
        self.primary_cover = Some(s.into());
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
    pub fn alt_titles<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = (S1, S2)>,
    ) -> Self {
        self.alt_titles = s
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn description<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = (S1, S2)>,
    ) -> Self {
        self.description = s
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn links<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = (S1, S2)>,
    ) -> Self {
        self.links = s
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn authors<A: Into<AuthorId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.authors = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn artists<A: Into<ArtistId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.artists = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.tags = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn publication_demographic(mut self, s: impl IntoIterator<Item = Demographic>) -> Self {
        self.publication_demographic = s.into_iter().collect();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateManga {
    pub version: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_rating: Option<ContentRating>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alt_titles: Vec<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<AuthorId>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub artists: Vec<ArtistId>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chapter: Option<ChapterId>,

    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub publication_demographic: HashSet<Demographic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_numbers_reset_on_new_volume: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cover: Option<CoverId>,
}

impl UpdateManga {
    pub fn new(version: usize) -> Self {
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
    pub fn last_chapter(mut self, s: impl Into<ChapterId>) -> Self {
        self.last_chapter = Some(s.into());
        self
    }
    pub fn primary_cover(mut self, s: impl Into<CoverId>) -> Self {
        self.primary_cover = Some(s.into());
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
    pub fn alt_titles<
        S1: std::fmt::Display,
        S2: std::fmt::Display,
        I: IntoIterator<Item = (S1, S2)>,
    >(
        mut self,
        s: impl IntoIterator<Item = I>,
    ) -> Self {
        self.alt_titles = s
            .into_iter()
            .map(|i| {
                i.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            })
            .collect();
        self
    }
    pub fn description<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = (S1, S2)>,
    ) -> Self {
        self.description = s
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn links<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = (S1, S2)>,
    ) -> Self {
        self.links = s
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn authors<A: Into<AuthorId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.authors = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn artists<A: Into<ArtistId>>(mut self, s: impl IntoIterator<Item = A>) -> Self {
        self.artists = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.tags = s.into_iter().map(|v| v.into()).collect();
        self
    }
    pub fn publication_demographic(mut self, s: impl IntoIterator<Item = Demographic>) -> Self {
        self.publication_demographic = s.into_iter().collect();
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_original_languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_ratings: Option<HashSet<ContentRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_groups: Option<Vec<GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_uploaders: Option<Vec<UserId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at_since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<ChapterInclude>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_empty_pages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_publish_at: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_url: Option<bool>,
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

    pub fn translated_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.translated_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn original_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.original_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn excluded_original_languages<S: std::fmt::Display>(
        mut self,
        s: impl IntoIterator<Item = S>,
    ) -> Self {
        self.excluded_original_languages = Some(s.into_iter().map(|v| v.to_string()).collect());
        self
    }
    pub fn excluded_groups<G: Into<GroupId>>(mut self, s: impl IntoIterator<Item = G>) -> Self {
        self.excluded_groups = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }
    pub fn excluded_uploaders<U: Into<UserId>>(mut self, s: impl IntoIterator<Item = U>) -> Self {
        self.excluded_uploaders = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item = ContentRating>) -> Self {
        self.content_ratings = Some(s.into_iter().collect());
        self
    }
    pub fn includes(mut self, s: impl IntoIterator<Item = ChapterInclude>) -> Self {
        self.includes = Some(s.into_iter().collect());
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item = (S, Order)>) -> Self {
        self.order = Some(s.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
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
        request.add_param_opt("translatedLanguage", self.translated_languages);
        request.add_param_opt("originalLanguage", self.original_languages);
        request.add_param_opt("excludedOriginalLanguage", self.excluded_original_languages);
        request.add_param_opt("contentRatings", self.content_ratings);
        request.add_param_opt("excludedGroups", self.excluded_groups);
        request.add_param_opt("excludedUploaders", self.excluded_uploaders);
        request.add_param_opt("createdAtSince", self.created_at_since);
        request.add_param_opt("updatedAtSince", self.updated_at_since);
        request.add_param_opt("publishAtSince", self.publish_at_since);
        request.add_param_opt("order", self.order);
        request.add_param_opt("includes", self.includes);

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

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RandomMangaFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<MangaInclude>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_ratings: Option<HashSet<ContentRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_tags: Option<Vec<TagId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_tags_mode: Option<TagMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags: Option<Vec<TagId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tags_mode: Option<TagMode>,
}

impl RandomMangaFilter {
    pub fn includes(mut self, s: impl IntoIterator<Item = MangaInclude>) -> Self {
        self.includes = Some(s.into_iter().collect());
        self
    }

    pub fn content_ratings(mut self, s: impl IntoIterator<Item = ContentRating>) -> Self {
        self.content_ratings = Some(s.into_iter().collect());
        self
    }

    pub fn included_tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.included_tags = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn included_tags_mode(mut self, s: TagMode) -> Self {
        self.included_tags_mode = Some(s);
        self
    }

    pub fn excluded_tags<T: Into<TagId>>(mut self, s: impl IntoIterator<Item = T>) -> Self {
        self.excluded_tags = Some(s.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn excluded_tags_mode(mut self, s: TagMode) -> Self {
        self.excluded_tags_mode = Some(s);
        self
    }
}

impl ExtendParams for RandomMangaFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("includes", self.includes);
        request.add_param_opt("contentRating", self.content_ratings);
        request.add_param_opt("includedTags", self.included_tags);
        request.add_param_opt(
            "includedTagsMode",
            self.included_tags_mode.map(|v| v.to_string()),
        );
        request.add_param_opt("excludedTags", self.excluded_tags);
        request.add_param_opt(
            "excludedTagsMode",
            self.excluded_tags_mode.map(|v| v.to_string()),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statuses<D> {
    pub statuses: D,
}
impl<D> IntoData<D> for Statuses<D> {
    fn into_data(self) -> D {
        self.statuses
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStatus {
    pub status: Status,
}
impl IntoData<Status> for DataStatus {
    fn into_data(self) -> Status {
        self.status
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DraftFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MangaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<MangaInclude>>,
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

    pub fn includes(mut self, s: impl IntoIterator<Item = MangaInclude>) -> Self {
        self.includes = Some(s.into_iter().collect());
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, s: impl IntoIterator<Item = (S, Order)>) -> Self {
        self.order = Some(s.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        self
    }
}

impl ExtendParams for DraftFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("includes", self.includes);
        request.add_param_opt("order", self.order);
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("state", self.state.map(|v| v.to_string()));
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaRelationAttributes {
    pub relation: Relation,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaRelation {
    pub id: Uuid,
    pub attributes: MangaRelationAttributes,
    pub relationships: Vec<Relationship>,
}
