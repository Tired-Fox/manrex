use serde::{Deserialize, Serialize};

use crate::Uuid;

use super::Relationship;


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadAttributes {
    pub replies_count: usize
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: Uuid,
    pub attributes: ThreadAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Resource {
    Manga,
    Group,
    Chapter,
}
