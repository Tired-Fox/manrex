use std::collections::BTreeMap;

use crate::{
    client::ExtendParams,
    uuid::{ReasonId, ReportId},
    Uuid,
};

use super::{Category, Order, Relationship};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportReasonAttributes {
    pub reason: BTreeMap<String, String>,
    pub details_required: bool,
    pub category: Category,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportReason {
    pub id: ReasonId,
    pub attributes: ReportReasonAttributes,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReportStatus {
    Waiting,
    Accepted,
    Refused,
    Autoresolved,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportAttributes {
    pub details: String,
    pub object_id: Uuid,
    pub status: ReportStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub id: ReportId,
    pub attributes: ReportReasonAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReport {
    pub category: Category,
    pub reason: String,
    pub object_id: Uuid,
    pub details: String,
}
impl CreateReport {
    pub fn new(
        category: Category,
        reason: impl std::fmt::Display,
        object_id: impl Into<Uuid>,
        details: impl std::fmt::Display,
    ) -> Self {
        Self {
            category,
            reason: reason.to_string(),
            object_id: object_id.into(),
            details: details.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReportInclude {
    Reason,
    User,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ReportFilter {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category: Option<Category>,
    pub reason_id: Option<ReasonId>,
    pub object_id: Option<Uuid>,
    pub status: Option<ReportStatus>,

    pub order: BTreeMap<String, Order>,
    pub includes: Vec<ReportInclude>,
}

impl ReportFilter {
    pub fn limit(mut self, state: usize) -> Self {
        self.limit = Some(state);
        self
    }

    pub fn offset(mut self, state: usize) -> Self {
        self.offset = Some(state);
        self
    }

    pub fn category(mut self, state: Category) -> Self {
        self.category = Some(state);
        self
    }

    pub fn status(mut self, state: ReportStatus) -> Self {
        self.status = Some(state);
        self
    }

    pub fn reason_id<R: Into<ReasonId>>(mut self, state: R) -> Self {
        self.reason_id = Some(state.into());
        self
    }

    pub fn object_id<U: Into<Uuid>>(mut self, state: U) -> Self {
        self.object_id = Some(state.into());
        self
    }

    pub fn order<S: std::fmt::Display>(
        mut self,
        includes: impl IntoIterator<Item = (S, Order)>,
    ) -> Self {
        self.order = includes
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self
    }

    pub fn include(mut self, includes: impl IntoIterator<Item = ReportInclude>) -> Self {
        self.includes.extend(includes);
        self
    }
}

impl ExtendParams for ReportFilter {
    fn extend_params(self, request: &mut crate::client::Request) {
        request.add_param_opt("limit", self.limit);
        request.add_param_opt("offset", self.offset);
        request.add_param_opt("category", self.category.map(|v| v.to_string()));
        request.add_param_opt("status", self.status.map(|v| v.to_string()));
        request.add_param_opt("reasonId", self.reason_id);
        request.add_param_opt("objectId", self.object_id);

        if !self.order.is_empty() {
            request.add_param("order", self.order);
        }

        if !self.includes.is_empty() {
            request.add_param("includes", self.includes);
        }
    }
}
