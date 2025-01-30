use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::IntoData;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics<D> {
    pub statistics: D,
}

impl<S: Ord> IntoData<BTreeMap<S, Comments>> for Statistics<BTreeMap<S, StatisticComments>> {
    fn into_data(self) -> BTreeMap<S, Comments> {
        self.statistics
            .into_iter()
            .map(|(k, v)| (k, v.comments))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticComments {
    pub comments: Comments,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    /// The id of the thead backing the comments for that entity on the MangaDex Forums.
    pub thread_id: usize,
    /// The number of replies on the MangaDex Forums thread backing this entity's comments.
    /// This excludes the initial comment that opens the thread, which is created by our systems.
    pub replies_count: usize,
}
