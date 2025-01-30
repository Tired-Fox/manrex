use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{client::ExtendParams, uuid::AuthorId};

use super::{Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AuthorInclude {
    Manga,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AuthorFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<AuthorInclude>>,
}

impl AuthorFilter {
    pub fn limit(mut self, state: usize) -> Self {
        self.limit = Some(state);
        self
    }

    pub fn offset(mut self, state: usize) -> Self {
        self.offset = Some(state);
        self
    }

    pub fn ids<A: Into<AuthorId>>(mut self, ids: impl IntoIterator<Item = A>) -> Self {
        self.ids = Some(ids.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn order<S: std::fmt::Display>(
        mut self,
        order: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order = Some(order.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        self
    }

    pub fn includes(mut self, includes: impl IntoIterator<Item = AuthorInclude>) -> Self {
        self.includes = Some(includes.into_iter().collect());
        self
    }
}

impl ExtendParams for AuthorFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("ids", self.ids);
        request.add_param_opt("order", self.order);
        request.add_param_opt("includes", self.includes);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    pub name: String,
    pub version: usize,

    pub image_url: Option<String>,
    /// Localization to target text map
    #[serde(default)]
    pub biography: BTreeMap<String, String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub links: BTreeMap<String, Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: AuthorId,
    pub attributes: AuthorAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuthor {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub biography: Option<BTreeMap<String, String>>,

    /* Links */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixiv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melon_book: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_box: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nico_video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fantia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumblr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weibo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl CreateAuthor {
    pub fn new(name: impl std::fmt::Display) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: impl std::fmt::Display) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn biography<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        biography: impl IntoIterator<Item=(S1, S2)>,
    ) -> Self {
        self.biography = Some(biography.into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect());
        self
    }

    pub fn twitter(mut self, s: impl std::fmt::Display) -> Self {
        self.twitter = Some(s.to_string());
        self
    }

    pub fn pixiv(mut self, s: impl std::fmt::Display) -> Self {
        self.pixiv = Some(s.to_string());
        self
    }

    pub fn melon_book(mut self, s: impl std::fmt::Display) -> Self {
        self.melon_book = Some(s.to_string());
        self
    }

    pub fn fan_box(mut self, s: impl std::fmt::Display) -> Self {
        self.fan_box = Some(s.to_string());
        self
    }

    pub fn booth(mut self, s: impl std::fmt::Display) -> Self {
        self.booth = Some(s.to_string());
        self
    }

    pub fn nico_video(mut self, s: impl std::fmt::Display) -> Self {
        self.nico_video = Some(s.to_string());
        self
    }

    pub fn skeb(mut self, s: impl std::fmt::Display) -> Self {
        self.skeb = Some(s.to_string());
        self
    }

    pub fn fantia(mut self, s: impl std::fmt::Display) -> Self {
        self.fantia = Some(s.to_string());
        self
    }

    pub fn tumblr(mut self, s: impl std::fmt::Display) -> Self {
        self.tumblr = Some(s.to_string());
        self
    }

    pub fn youtube(mut self, s: impl std::fmt::Display) -> Self {
        self.youtube = Some(s.to_string());
        self
    }

    pub fn weibo(mut self, s: impl std::fmt::Display) -> Self {
        self.weibo = Some(s.to_string());
        self
    }

    pub fn naver(mut self, s: impl std::fmt::Display) -> Self {
        self.naver = Some(s.to_string());
        self
    }

    pub fn website(mut self, s: impl std::fmt::Display) -> Self {
        self.website = Some(s.to_string());
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthor {
    pub name: String,
    pub version: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub biography: Option<BTreeMap<String, String>>,

    /* Links */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixiv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melon_book: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_box: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nico_video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fantia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumblr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weibo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl UpdateAuthor {
    pub fn new(name: impl std::fmt::Display, version: usize) -> Self {
        Self {
            name: name.to_string(),
            version,
            ..Default::default()
        }
    }

    pub fn name(mut self, name: impl std::fmt::Display) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn version(mut self, s: usize) -> Self {
        self.version = s;
        self
    }

    pub fn biography<S1: std::fmt::Display, S2: std::fmt::Display>(
        mut self,
        biography: impl IntoIterator<Item=(S1, S2)>,
    ) -> Self {
        self.biography = Some(biography.into_iter().map(|(a, b)| (a.to_string(), b.to_string())).collect());
        self
    }

    pub fn twitter(mut self, s: impl std::fmt::Display) -> Self {
        self.twitter = Some(s.to_string());
        self
    }

    pub fn pixiv(mut self, s: impl std::fmt::Display) -> Self {
        self.pixiv = Some(s.to_string());
        self
    }

    pub fn melon_book(mut self, s: impl std::fmt::Display) -> Self {
        self.melon_book = Some(s.to_string());
        self
    }

    pub fn fan_box(mut self, s: impl std::fmt::Display) -> Self {
        self.fan_box = Some(s.to_string());
        self
    }

    pub fn booth(mut self, s: impl std::fmt::Display) -> Self {
        self.booth = Some(s.to_string());
        self
    }

    pub fn nico_video(mut self, s: impl std::fmt::Display) -> Self {
        self.nico_video = Some(s.to_string());
        self
    }

    pub fn skeb(mut self, s: impl std::fmt::Display) -> Self {
        self.skeb = Some(s.to_string());
        self
    }

    pub fn fantia(mut self, s: impl std::fmt::Display) -> Self {
        self.fantia = Some(s.to_string());
        self
    }

    pub fn tumblr(mut self, s: impl std::fmt::Display) -> Self {
        self.tumblr = Some(s.to_string());
        self
    }

    pub fn youtube(mut self, s: impl std::fmt::Display) -> Self {
        self.youtube = Some(s.to_string());
        self
    }

    pub fn weibo(mut self, s: impl std::fmt::Display) -> Self {
        self.weibo = Some(s.to_string());
        self
    }

    pub fn naver(mut self, s: impl std::fmt::Display) -> Self {
        self.naver = Some(s.to_string());
        self
    }

    pub fn website(mut self, s: impl std::fmt::Display) -> Self {
        self.website = Some(s.to_string());
        self
    }
}
