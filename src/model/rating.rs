use serde::{Deserialize, Serialize};

use super::IntoData;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    rating: usize,
    created_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings<D> {
    ratings: D
}
impl<D> IntoData<D> for Ratings<D> {
    fn into_data(self) -> D {
        self.ratings
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    chapter_id: String,
    read_date: String,
}
