use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{client::ExtendParams, uuid::ClientId};

use super::{Order, Relationship};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum ApiClientState {
    Requested,
    Approved,
    Rejected,
    Autoapproved,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiClientAttributes {
    pub name: String,
    pub description: Option<String>,
    pub profile: String,
    pub external_client_id: Option<String>,
    pub is_active: bool,
    pub state: ApiClientState,
    pub created_at: String,
    pub updated_at: String,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiClient {
    pub id: ClientId,
    pub attributes: ApiClientAttributes,
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
    pub state: Option<ApiClientState>,
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

    pub fn state(self, state: ApiClientState) -> Self {
        Self {
            state: Some(state),
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
        request.add_param_opt("state", self.state.map(|v| v.to_string()));
        request.add_param_opt("order", self.order);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum::Display)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum ClientInclude {
    Creator,
}
