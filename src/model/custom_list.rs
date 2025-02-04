use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{client::ExtendParams, ListId, MangaId};

use super::{Order, Relationship, Visibility};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ListInclude {
    Manga,
    User,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: Visibility,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomList {
    pub id: ListId,
    pub attributes: CustomListAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
}
impl ClientFilter {
    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    pub fn offset(self, offset: usize) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }

    pub fn order<S: std::fmt::Display>(self, order: impl IntoIterator<Item = (S, Order)>) -> Self {
        Self {
            order: Some(order.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
            ..self
        }
    }
}
impl ExtendParams for ClientFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("order", self.order);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomList {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga: Option<HashSet<MangaId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<usize>,
}
impl CreateCustomList {
    pub fn new(self, name: impl std::fmt::Display) -> Self {
        Self {
            name: name.to_string(),
            visibility: None,
            manga: None,
            version: None,
        }
    }

    pub fn visibility(self, vis: Visibility) -> Self {
        Self {
            visibility: Some(vis),
            ..self
        }
    }

    pub fn manga<S: Into<MangaId>>(self, manga: impl IntoIterator<Item=S>) -> Self {
        Self {
            manga: Some(manga.into_iter().map(|m| m.into()).collect()),
            ..self
        }
    }

    pub fn version(self, version: usize) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCustomList {
    pub version: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga: Option<HashSet<MangaId>>,
}
impl UpdateCustomList {
    pub fn new(self, version: usize) -> Self {
        Self {
            version,
            name: None,
            visibility: None,
            manga: None,
        }
    }

    pub fn visibility(self, vis: Visibility) -> Self {
        Self {
            visibility: Some(vis),
            ..self
        }
    }

    pub fn manga<S: Into<MangaId>>(self, manga: impl IntoIterator<Item=S>) -> Self {
        Self {
            manga: Some(manga.into_iter().map(|m| m.into()).collect()),
            ..self
        }
    }

    pub fn name(self, name: impl std::fmt::Display) -> Self {
        Self {
            name: Some(name.to_string()),
            ..self
        }
    }
}
