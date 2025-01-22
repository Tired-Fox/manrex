use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::client::ExtendParams;

use super::{Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all="camelCase")]
#[strum(serialize_all="snake_case")]
pub enum ApiClientState {
    Requested,
    Approved,
    Rejected,
    Autoapproved
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ApiClientAttributes {
    pub name: String,
    pub description: Option<String>,
    pub profile: String,
    pub external_client_id: Option<String>,
    pub is_active: bool,
    pub state: ApiClientState,
    pub created_at: String,
    pub updated_at: String,
    pub version: usize
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ApiClient {
    pub id: String,
    pub attributes: ApiClientAttributes,
    #[serde(default)]
    pub relationships: Vec<Relationship>
}

#[derive(Default, Debug, PartialEq)]
pub struct ClientFilter {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub state: Option<ApiClientState>,
    pub orders: BTreeMap<String, Order>
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

    pub fn state(self, state: ApiClientState) -> Self {
        Self {
            state: Some(state),
            ..self
        }
    }

    pub fn orders<S: std::fmt::Display>(self, order: impl IntoIterator<Item=(S, Order)>) -> Self {
        Self {
            orders: order.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
            ..self
        }
    }
}
impl ExtendParams for ClientFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        if let Some(limit) = self.limit {
            request.add_param("limit", limit);
        }
        if let Some(offset) = self.offset {
            request.add_param("offset", offset);
        }
        if let Some(state) = self.state {
            request.add_param("state", state.to_string());
        }
        if !self.orders.is_empty() {
            request.add_param("order", self.orders);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, strum::Display)]
#[serde(rename_all="camelCase")]
#[strum(serialize_all="snake_case")]
pub enum ClientInclude {
    Creator
}
