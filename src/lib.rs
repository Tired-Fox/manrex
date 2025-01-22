mod error;

mod client;
pub mod model;

use std::future::Future;

pub use error::Error;
pub use client::{Client, auth};

pub trait JsonWithErrorPath {
    fn json_with_error_path<T: serde::de::DeserializeOwned>(self) -> impl Future<Output=Result<T, Error>>;
}

impl JsonWithErrorPath for reqwest::Response {
    async fn json_with_error_path<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let full = self.bytes().await?;
        Ok(serde_json_path_to_error::from_slice(&full)?)
    }
}
