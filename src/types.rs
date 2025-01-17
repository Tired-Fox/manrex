#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Demographic {
    Shounen,
    Shoujo,
    Josei,
    Seinen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum ReadingStatus {
    Reading,
    OnHold,
    PlanToRead,
    Dropped,
    ReReading,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Ordering {
    Asc,
    Dsc
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Relationship {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
    ScanlationGroup,
    Tag,
    User,
    CustomList
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="snake_case")]
pub enum Related {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="lowercase")]
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
    #[serde(rename="ROLE_ADMIN")]
    Admin,
    /// Banned
    #[serde(rename="ROLE_BANNED")]
    Banned,
    /// Helpers contributing by filling in missing information (Description, External Links) on Manga pages on MangaDex
    #[serde(rename="ROLE_CONTRIBUTOR")]
    Contributor,
    /// Designer
    #[serde(rename="ROLE_DESIGNER")]
    Designer,
    /// MangaDex site developers
    #[serde(rename="ROLE_DEVELOPER")]
    Developer,
    /// Moderates the forum
    #[serde(rename="ROLE_FORUM_MODERATOR")]
    ForumModerator,
    #[serde(rename="ROLE_GLOBAL_MODERATOR")]
    GlobalModerator,
    /// Leaders of active groups on MangaDex
    #[serde(rename="ROLE_GROUP_LEADER")]
    GroupLeader,
    /// Member of a group
    #[serde(rename="ROLE_GROUP_MEMBER")]
    GroupMember,
    /// Users viewing the site without being logged in
    #[serde(rename="ROLE_GUEST")]
    Guest,
    /// A normal account
    #[serde(rename="ROLE_MEMBER")]
    Member,
    /// Involved with the MangaDex@Home project
    #[serde(rename="ROLE_MD_AT_HOME")]
    MdAtHome,
    /// Uploaded 500 or more chapters to MangaDex
    #[serde(rename="ROLE_POWER_UPLOADER")]
    PowerUploader,
    /// Manages social media
    #[serde(rename="ROLE_PUBLIC_RELATIONS")]
    PublicRelations,
    /// Staff
    #[serde(rename="ROLE_STAFF")]
    Staff,
    /// Accounts that haven't had their email address verified yet
    #[serde(rename="ROLE_UNVERIFIED")]
    Unverified,
    /// A normal account
    #[serde(rename="ROLE_USER")]
    User,
    /// Important people that in one way or another helped MangaDex
    #[serde(rename="ROLE_VIP")]
    Vip,
}
