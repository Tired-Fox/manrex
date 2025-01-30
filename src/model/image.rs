use std::{path::PathBuf, pin::Pin};

use bytes::Bytes;
use chrono::{DateTime, Local};
use futures_util::Stream;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};

use crate::{bail, client::{Request, CLIENT_NAME, CLIENT_VERSION}, Error};

type ByteStream = Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>> + Send>>;

pub struct Image {
    /// URL where to fetch the image
    pub url: String,
    /// A timestamp when the url expires, if at all
    pub expires: Option<DateTime<Local>>,
    /// Name of the image file
    pub file_name: Option<PathBuf>,
}

impl Image {
    pub async fn fetch(&self) -> Result<(Option<String>, ByteStream), Error> {
        if self.expires.is_some_and(|v| Local::now() >= v) {
            bail!("image url has expired and is no longer valid")
        }

        let res = Request::get(&self.url)
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await?;
            Err(Error::http(status, body))
        } else {
            Ok((
                res.headers().get(CONTENT_TYPE).map(|v| v.to_str().unwrap().to_string()),
                Box::pin(res.bytes_stream())
            ))
        }
    }
}
