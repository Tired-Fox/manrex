use std::collections::BTreeMap;

use serde::Deserialize;

use crate::client::ExtendParams;

use super::{Order, Relationship};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UserFilter {
    limit: Option<usize>,
    offset: Option<usize>,
    ids: Vec<String>,
    username: Option<String>,
    order: BTreeMap<String, Order>,
}

impl UserFilter {
    pub fn limit(mut self, state: usize) -> Self {
        self.limit = Some(state);
        self
    }

    pub fn offset(mut self, state: usize) -> Self {
        self.offset = Some(state);
        self
    }

    pub fn username(mut self, state: impl std::fmt::Display) -> Self {
        self.username = Some(state.to_string());
        self
    }

    pub fn ids<S: std::fmt::Display>(mut self, ids: impl IntoIterator<Item=S>) -> Self {
        self.ids.extend(ids.into_iter().map(|v| v.to_string()));
        self
    }

    pub fn order<S: std::fmt::Display>(mut self, orders: impl IntoIterator<Item=(S, Order)>) -> Self {
        self.order.extend(orders.into_iter().map(|(k, v)| (k.to_string(), v)));
        self
    }
}

impl ExtendParams for UserFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("username", self.username);
        if !self.ids.is_empty() {
            request.add_param("ids", self.ids);
        }

        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    username: String,
    roles: Vec<String>,
    version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    attributes: UserAttributes,
    relationships: Vec<Relationship>,
}
