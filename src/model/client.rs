use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::client::ExtendParams;

use super::{Order, Related};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ApiClientState {
    Requested,
    Approved,
    Rejected,
    Autoapproved
}
impl std::fmt::Display for ApiClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::Autoapproved => write!(f, "autoapproved")
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RelationshipAttributes {
    description: Option<String>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Relationship {
    id: String,
    related: Option<Related>,
    attributes: Option<RelationshipAttributes>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ApiClientAttributes {
    name: String,
    description: Option<String>,
    profile: String,
    external_client_id: Option<String>,
    is_active: bool,
    state: ApiClientState,
    created_at: String,
    updated_at: String,
    version: usize
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ApiClient {
    pub id: String,
    pub attributes: ApiClientAttributes,
    pub relationships: Vec<Relationship>
}

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct ClientFilter {
    limit: Option<usize>,
    offset: Option<usize>,
    state: Option<ApiClientState>,
    order: Option<BTreeMap<String, Order>>
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

    pub fn order<S: std::fmt::Display>(self, order: impl IntoIterator<Item=(S, Order)>) -> Self {
        Self {
            order: Some(order.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
            ..self
        }
    }
}
impl ExtendParams for ClientFilter {
    fn extend_params(&self, request: &mut crate::client::Request) {
        if let Some(limit) = self.limit {
            request.add_param("limit", limit);
        }
        if let Some(offset) = self.offset {
            request.add_param("offset", offset);
        }
        if let Some(state) = self.state {
            request.add_param("state", state);
        }
        if let Some(order) = self.order.as_ref() {
            for (name, order) in order.iter() {
                request.add_param(name, *order);
            }
        }
    }
}
