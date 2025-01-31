mod error;
mod uuid;

mod client;
pub mod model;

use std::future::Future;

pub use client::{auth, Client};
pub use error::Error;
pub use uuid::*;

pub trait JsonWithErrorPath {
    fn json_with_error_path<T: serde::de::DeserializeOwned>(
        self,
    ) -> impl Future<Output = Result<T, Error>>;
}

impl JsonWithErrorPath for reqwest::Response {
    async fn json_with_error_path<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let mut full = self.bytes().await?.to_vec();
        if full.is_empty() {
            full.extend(b"null");
        }
        Ok(serde_json::from_slice(&full)?)
    }
}
