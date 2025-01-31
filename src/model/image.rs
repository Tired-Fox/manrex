use std::pin::Pin;

use bytes::Bytes;
use chrono::{DateTime, Local};
use futures_util::{Stream, StreamExt};
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use tokio::{io::AsyncWriteExt, time::Instant};

use crate::{bail, client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION}, error::ResponseToError, Error};

use super::at_home::AtHomeImageReport;

type ByteStream = Pin<Box<dyn Stream<Item=reqwest::Result<Bytes>> + Send + Sync>>;

/// An representation of an image from a `MangaDex` server
///
/// Some images are bound by a time limit based on what is cached for a given server.
/// These images have an `experation` and may return an error when attempting to fetch them.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// URL where to fetch the image
    pub(crate) url: String,
    /// Name of the image file
    pub(crate) file_name: String,
    /// A timestamp when the url expires, if at all
    ///
    /// This also marks the image as being a source of a `MangaDex@Home` server.
    pub(crate) expires: Option<DateTime<Local>>,
}

impl Image {
    /// Image file name from the server
    pub fn file_name(&self) -> &str {
        &self.file_name
    } 

    /// Whether this image is still valid given the parent [`AtHome`][model::at_home::AtHome]'s
    /// expiration date and time
    pub fn expired(&self) -> bool {
        self.expires.as_ref().map(|v| v <= &Local::now()).unwrap_or_default()
    } 

    /// Fetch the entire image
    ///
    /// This endpoint will also automatically report the success or failure to MangaDex@Home image.
    pub async fn retrieve(&self) -> Result<ImageData, Error> {
        let mut image_stream = self.fetch().await?;

        let mut bytes = Vec::new();
        let start = Instant::now();

        while let Some(chunk) = image_stream.stream.next().await {
            match chunk {
                Ok(chunk) => bytes.extend(chunk),
                Err(err) => {
                    image_stream.report(false, bytes.len(), start.elapsed().as_millis()).await?;
                    return Err(Error::from(err));
                }
            }
        }

        image_stream.report(true, bytes.len(), start.elapsed().as_millis()).await?;

        Ok(ImageData {
            mime: image_stream.mime,
            data: bytes,
            cached: image_stream.cached
        })
    }

    /// Fetch and stream the image
    ///
    /// If an error occurs when streaming the bytes or if the image was fetched successfully
    /// then a call [`ImageStream::report`] should be made. This will report success or failures
    /// for `MangaDex@Home` servers so that `MangaDex` can manage the health status of the server where
    /// the image is from.
    ///
    /// # Example
    ///
    /// ```
    /// // Easiest way to stream bytes
    ///
    /// use manrex::{auth::{Credentials, OAuth}, Client, model::manga::MangaInclude};
    /// use futures_util::StreamExt;
    ///
    /// let client = Client::new(OAuth::new(Credentials::from_env()?));
    /// let manga = client.get_manga("6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc", [MangaInclude::CoverArt]).await?;
    /// let image = manga.get_cover_art(None);
    ///
    /// // Fetch and stream the image
    /// let mut stream = image.fetch().await?;
    /// // Determine the file extension from the mime
    /// let ext = match stream.mime.as_str() {
    ///     "image/jpeg" => ".jpg",
    ///     "image/webp" => ".webp",
    ///     "image/avif" => ".avif",
    ///     _ => ".png",
    /// };
    ///
    /// let mut file = tokio::fs::OpenOtions::new()
    ///     .write(true)
    ///     .truncate(true)
    ///     .create(true)
    ///     .open(format!("{}{ext}", image.file_name()))
    ///     .await?;
    /// 
    /// stream.stream_to(&mut file).await?;
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// // Manually streaming the image bytes
    ///
    /// use manrex::{auth::{Credentials, OAuth}, Client, model::manga::MangaInclude};
    /// use futures_util::StreamExt;
    ///
    /// let client = Client::new(OAuth::new(Credentials::from_env()?));
    /// let manga = client.get_manga("6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc", [MangaInclude::CoverArt]).await?;
    ///
    /// let image = manga.get_cover_art(None);
    ///
    /// let mut file = tokio::fs::OpenOtions::new()
    ///     .write(true)
    ///     .truncate(true)
    ///     .create(true)
    ///     .open("test.png")
    ///     .await?;
    ///
    /// let mut total_bytes = 0usize;
    /// let mut image_stream = image.fetch().await?;
    /// let start = tokio::time::Instant::now();
    ///
    /// while let Some(chunk) = image_stream.stream.next().await {
    ///     match chunk {
    ///         Ok(chunk) => {
    ///             total_bytes += chunk.len();
    ///             file.write_all(chunk).await?
    ///         },
    ///         Err(err) => {
    ///             image_stream.report(false, total_bytes, start.elapsed().as_millis()).await?;
    ///             break;
    ///         }
    ///     }
    /// }
    ///
    /// image_stream.report(true, total_bytes, start.elapsed().as_millis()).await?;
    /// ```
    pub async fn fetch(&self) -> Result<ImageStream, Error> {
        if self.expired() {
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
            let mime = res.headers().get(CONTENT_TYPE).map(|v| v.to_str().unwrap().to_string()).expect("failed to get Content-Type for image");
            let cached = res.headers().get("X-Cache").map(|v| v.to_str().unwrap().starts_with("HIT")).unwrap_or_default();
            Ok(
                ImageStream {
                    url: self.url.clone(),
                    report: self.expires.is_some(),
                    stream: Box::pin(res.bytes_stream()),
                    mime,
                    cached 
                }
            )
        }
    }
}

/// Response Data for a streamed image.
pub struct ImageStream {
    url: String,
    report: bool,
    pub mime: String,
    pub stream: ByteStream,
    pub cached: bool,
}

impl ImageStream {
    /// Create an AtHomeReport that can be passed to [Client::at_home_image_report][client::Client::at_home_image_report]
    ///
    /// # Arguments
    ///
    /// - `success`: Whether the image was retrieved successfully
    /// - `bytes`: Total number of bytes retrieved for the image. This includes incomplete images
    ///     due to errors.
    /// - `duration`: How long it took to retrieved the image. **NOT** Time to first byte (TTFB)
    pub async fn report(&self, succes: bool, bytes: usize, duration: u128) -> Result<(), Error> {
        if self.report {
            let res = Request::post((MangaDex::ApiNetwork, Endpoint::Report))
                .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
                .json(&AtHomeImageReport {
                    url: self.url.clone(),
                    succes,
                    cached: self.cached,
                    bytes,
                    duration
                })
                .send()
            .await?;

            ResponseToError::<()>::manga_dex_response_empty(res).await?;
        }
        Ok(())
    }

    /// Consume the image stream and write it to an async writable buffer
    ///
    /// # Arguments
    ///
    /// - `buffer`: Something that supports [`tokio::io::AsyncWriteExt`]
    pub async fn stream_to<B: AsyncWriteExt + Unpin>(mut self, buffer: &mut B) -> Result<(), Error> {
        let mut total_bytes: usize = 0;
        let start = Instant::now();

        while let Some(chunk) = self.stream.next().await {
            match chunk {
                Ok(chunk) => {
                    total_bytes = total_bytes.saturating_add(chunk.len());
                    buffer.write_all(chunk.as_ref()).await?
                },
                Err(err) => {
                    self.report(false, total_bytes, start.elapsed().as_millis()).await?;
                    return Err(Error::from(err));
                }
            }
        }

        self.report(true, total_bytes, start.elapsed().as_millis()).await?;
        Ok(())
    }
}

/// Response Data for a full image.
pub struct ImageData {
    /// The `Content-Type` header value
    pub mime: String,
    pub data: Vec<u8>,
    /// Whether the `X-Cache` header was present and started with `HIT`
    pub cached: bool,
}
