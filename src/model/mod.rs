use author::AuthorAttributes;
use chapter::ChapterAttributes;
use cover::CoverAttributes;
use custom_list::CustomListAttributes;
use manga::{MangaAttributes, TagAttributes};
use scanlation_group::ScanlationGroupAttributes;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::MangaDexError, Error, Uuid};

pub mod at_home;
pub mod author;
pub mod chapter;
pub mod client;
pub mod cover;
pub mod custom_list;
pub mod forum;
mod image;
pub mod manga;
pub mod rating;
pub mod report;
pub mod scanlation_group;
pub mod settings;
pub mod statistics;
pub mod upload;
pub mod user;

pub use image::Image;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "result")]
pub enum MangaDexResponse<D> {
    Ok(D),
    Error { errors: Vec<MangaDexError> },
}

impl<D> MangaDexResponse<D> {
    pub fn ok(self) -> Result<D, Error> {
        match self {
            Self::Ok(data) => Ok(data),
            Self::Error { errors } => Err(Error::group(errors)),
        }
    }
}

/// Convert the parsed response into the expected data
pub(crate) trait IntoData<D> {
    fn into_data(self) -> D;
}

impl<D> IntoData<D> for D {
    fn into_data(self) -> D {
        self
    }
}

#[derive(Deserialize, Serialize)]
pub struct Paginated<D> {
    pub data: Vec<D>,
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
}

impl<D> IntoData<Vec<D>> for Paginated<D> {
    fn into_data(self) -> Vec<D> {
        self.data
    }
}

#[derive(Deserialize, Serialize)]
pub struct Data<D> {
    data: D,
}
impl<D> IntoData<D> for Data<D> {
    fn into_data(self) -> D {
        self.data
    }
}

impl<D: std::fmt::Debug> std::fmt::Debug for Paginated<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paginated")
            .field("data", &self.data)
            .field("limit", &self.limit)
            .field("offset", &self.offset)
            .field("total", &self.total)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, strum::EnumIs)]
#[serde(tag = "type", content = "attributes", rename_all = "snake_case")]
pub enum RelationshipAttributes {
    Manga(Option<MangaAttributes>),
    Chapter(Option<ChapterAttributes>),
    CoverArt(Option<CoverAttributes>),
    Author(Option<AuthorAttributes>),
    Artist(Option<Value>),
    ScanlationGroup(Option<ScanlationGroupAttributes>),
    Tag(Option<TagAttributes>),
    User(Option<Value>),
    CustomList(Option<CustomListAttributes>),
    #[serde(untagged)]
    Other(String),
}

impl RelationshipAttributes {
    pub fn as_manga(self) -> Option<MangaAttributes> {
        match self {
            Self::Manga(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as MangaAttributes"),
        }
    }

    pub fn as_chapter(self) -> Option<ChapterAttributes> {
        match self {
            Self::Chapter(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as ChapterAttributes"),
        }
    }

    pub fn as_cover_art(self) -> Option<CoverAttributes> {
        match self {
            Self::CoverArt(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as CoverArtAttributes"),
        }
    }

    pub fn as_author(self) -> Option<AuthorAttributes> {
        match self {
            Self::Author(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as AuthorAttributes"),
        }
    }

    pub fn as_artist(self) -> Option<Value> {
        match self {
            Self::Artist(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as ArtistAttributes"),
        }
    }

    pub fn as_scanlation_group(self) -> Option<ScanlationGroupAttributes> {
        match self {
            Self::ScanlationGroup(c) => c,
            _ => {
                unreachable!("failed to unwrap RelationshipAttributes as ScanlationGroupAttributes")
            }
        }
    }

    pub fn as_tag(self) -> Option<TagAttributes> {
        match self {
            Self::Tag(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as TagAttributes"),
        }
    }

    pub fn as_user(self) -> Option<Value> {
        match self {
            Self::User(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as UserAttributes"),
        }
    }

    pub fn as_custom_list(self) -> Option<CustomListAttributes> {
        match self {
            Self::CustomList(c) => c,
            _ => unreachable!("failed to unwrap RelationshipAttributes as CustomListAttributes"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: Uuid,
    pub related: Option<Relation>,
    #[serde(flatten)]
    pub attributes: Option<RelationshipAttributes>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Demographic {
    Shounen,
    Shoujo,
    Josei,
    Seinen,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Category {
    Manga,
    Chapter,
    ScanlationGroup,
    User,
    Author,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TagGroup {
    Content,
    Format,
    Genre,
    Theme,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MangaState {
    Draft,
    Submitted,
    Published,
    Rejected,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReadingStatus {
    Reading,
    OnHold,
    PlanToRead,
    Dropped,
    ReReading,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum TagMode {
    And,
    Or,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Ordering {
    Asc,
    Dsc,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Visibility {
    Public,
    Private,
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
//#[serde(rename_all="snake_case")]
//pub enum Relationship {
//    Manga,
//    Chapter,
//    CoverArt,
//    Author,
//    Artist,
//    ScanlationGroup,
//    Tag,
//    User,
//    CustomList
//}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Relation {
    /// A monochrome variant of this manga
    Monochrome,
    /// A colored variant of this manga
    Colored,
    /// The original version of this manga before its official serialization
    Preserialization,
    /// The official serialization of this manga
    Serialization,
    /// The previous entry in the same series
    Prequel,
    /// The next entry in the same series
    Sequel,
    /// The original narrative this manga is based on
    MainStory,
    /// A side work contemporaneous with the narrative of this manga
    SideStory,
    /// The original work this spin-off manga has been adapted from
    AdaptedFrom,
    /// An official derivative work based on this manga
    SpinOff,
    /// The original work this self-published derivative manga is based on
    BasedOn,
    /// A self-published derivative work based on this manga
    Doujinshi,
    /// A manga based on the same intellectual property as this manga
    SameFranchise,
    /// A manga taking place in the same fictional world as this manga
    SharedUniverse,
    /// An alternative take of the story in this manga
    AlternateStory,
    /// A different version of this manga with no other specific distinction
    AlternateVersion,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum MangaLinks {
    AL,
    AP,
    BW,
    MU,
    NU,
    KT,
    AMZ,
    EBJ,
    MAL,
    CDJ,
    Raw,
    EngTL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Role {
    /// MangaDex admins
    #[serde(rename = "ROLE_ADMIN")]
    Admin,
    /// Banned
    #[serde(rename = "ROLE_BANNED")]
    Banned,
    /// Helpers contributing by filling in missing information (Description, External Links) on Manga pages on MangaDex
    #[serde(rename = "ROLE_CONTRIBUTOR")]
    Contributor,
    /// Designer
    #[serde(rename = "ROLE_DESIGNER")]
    Designer,
    /// MangaDex site developers
    #[serde(rename = "ROLE_DEVELOPER")]
    Developer,
    /// Moderates the forum
    #[serde(rename = "ROLE_FORUM_MODERATOR")]
    ForumModerator,
    #[serde(rename = "ROLE_GLOBAL_MODERATOR")]
    GlobalModerator,
    /// Leaders of active groups on MangaDex
    #[serde(rename = "ROLE_GROUP_LEADER")]
    GroupLeader,
    /// Member of a group
    #[serde(rename = "ROLE_GROUP_MEMBER")]
    GroupMember,
    /// Users viewing the site without being logged in
    #[serde(rename = "ROLE_GUEST")]
    Guest,
    /// A normal account
    #[serde(rename = "ROLE_MEMBER")]
    Member,
    /// Involved with the MangaDex@Home project
    #[serde(rename = "ROLE_MD_AT_HOME")]
    MdAtHome,
    /// Uploaded 500 or more chapters to MangaDex
    #[serde(rename = "ROLE_POWER_UPLOADER")]
    PowerUploader,
    /// Manages social media
    #[serde(rename = "ROLE_PUBLIC_RELATIONS")]
    PublicRelations,
    /// Staff
    #[serde(rename = "ROLE_STAFF")]
    Staff,
    /// Accounts that haven't had their email address verified yet
    #[serde(rename = "ROLE_UNVERIFIED")]
    Unverified,
    /// A normal account
    #[serde(rename = "ROLE_USER")]
    User,
    /// Important people that in one way or another helped MangaDex
    #[serde(rename = "ROLE_VIP")]
    Vip,
}
