use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{client::ExtendParams, uuid::UserId};

use super::{Order, Relationship};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UserFilter {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub ids: Vec<UserId>,
    pub username: Option<String>,
    pub order: BTreeMap<String, Order>,
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

    pub fn ids<U: Into<UserId>>(mut self, ids: impl IntoIterator<Item = U>) -> Self {
        self.ids = ids.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn order<S: std::fmt::Display>(
        mut self,
        orders: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order
            .extend(orders.into_iter().map(|(k, v)| (k.to_string(), v)));
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
    pub username: String,
    pub roles: Vec<String>,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub attributes: UserAttributes,
    pub relationships: Vec<Relationship>,
}
