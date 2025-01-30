use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{client::ExtendParams, uuid::UserId};

use super::{Order, Relationship};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<UserId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<BTreeMap<String, Order>>,
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
        self.ids = Some(ids.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn order<S: std::fmt::Display>(
        mut self,
        orders: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order = Some(orders.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        self
    }
}

impl ExtendParams for UserFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("username", self.username);
        request.add_param_opt("ids", self.ids);
        request.add_param_opt("order", self.order);
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    pub username: String,
    pub roles: Vec<String>,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub attributes: UserAttributes,
    pub relationships: Vec<Relationship>,
}
