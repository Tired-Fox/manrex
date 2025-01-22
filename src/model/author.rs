use std::collections::BTreeMap;

use serde::{Deserialize};

use crate::client::ExtendParams;

use super::{Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub enum AuthorInclude {
    Manga,
}


#[derive(Default, Debug, Clone, PartialEq)]
pub struct AuthorFilter {
    limit: Option<usize>,
    offset: Option<usize>,
    ids: Vec<String>,
    name: Option<String>,
    orders: BTreeMap<String, Order>,
    includes: Vec<AuthorInclude>,
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

    pub fn id(mut self, state: impl std::fmt::Display) -> Self {
        self.ids.push(state.to_string());
        self
    }

    pub fn ids<S: std::fmt::Display>(mut self, ids: impl IntoIterator<Item=S>) -> Self {
        self.ids.extend(ids.into_iter().map(|v| v.to_string()));
        self
    }

    pub fn order(mut self, key: impl std::fmt::Display, order: Order) -> Self {
        self.orders.insert(key.to_string(), order);
        self
    }

    pub fn orders<S: std::fmt::Display>(mut self, orders: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.orders.extend(orders.into_iter().map(|(k, v)| (k.to_string(), v)));
        self
    }

    pub fn include(mut self, state: AuthorInclude) -> Self {
        self.includes.push(state);
        self
    }

    pub fn includes(mut self, includes: impl IntoIterator<Item=AuthorInclude>) -> Self {
        self.includes.extend(includes);
        self
    }
}

impl ExtendParams for AuthorFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }

        if !self.orders.is_empty() {
            request.add_param("order", self.orders);
        }

        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    name: String,
    version: usize,

    image_url: Option<String>,
    /// Localization to target text map
    #[serde(default)]
    biography: BTreeMap<String, String>,
    created_at: Option<String>,
    updated_at: Option<String>,

    #[serde(flatten)]
    links: BTreeMap<String, Option<String>>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    id: String,
    attributes: AuthorAttributes,
    #[serde(default)]
    relationships: Vec<Relationship>,
}
