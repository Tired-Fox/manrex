use std::future::Future;

use reqwest::{header::ToStrError, StatusCode};
use serde::de::DeserializeOwned;

use crate::{model::{IntoData, MangaDexResponse}, JsonWithErrorPath};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MangaDexError {
    id: String,
    status: usize,
    title: String,
    detail: Option<String>,
    context: Option<String>,
}

pub enum Error {
    Authorization,

    Http(StatusCode, String),

    MangaDex(MangaDexError),

    Group(Vec<Error>),
    Validation {
        name: String,
        expect: String,
        actual: String,
    },
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Self::custom(msg)
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Self::custom(msg)
    }
}

impl std::error::Error for Error {}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Error {
    pub fn custom(msg: impl std::fmt::Display) -> Self {
        Self::Custom(msg.to_string())
    }
    pub fn http(status: StatusCode, msg: impl std::fmt::Display) -> Self {
        Self::Http(status, msg.to_string())
    }
    pub fn group<E: Into<Self>>(errors: impl IntoIterator<Item=E>) -> Self {
        Self::Group(errors.into_iter().map(|v| v.into()).collect())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorization => write!(f, "attempt to call an authorized endpoint with an unauthorized client"),
            Self::Custom(msg) => write!(f, "{msg}"),
            Self::Http(status, msg) => write!(f, "http [{}] {msg}", status.as_u16()),
            Self::Validation { name, expect, actual } => write!(f, "invalid paramter '{name}': expected {expect}, but got {actual}"),
            Self::Group(errors) => {
                write!(f, "Error Group:")?;
                for error in errors {
                    write!(f, "  {error}")?;
                }
                Ok(())
            },
            Self::MangaDex(MangaDexError { id, status, title, detail, context }) => {
                write!(f, "[{id}] {status}:{title}")?;
                if let Some(detail) = detail {
                    write!(f, " {detail}")?;
                }
                if let Some(ctx) = context {
                    write!(f, " @ {ctx}")?;
                }
                Ok(())
            },
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::custom(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::custom(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::custom(value)
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<ToStrError> for Error {
    fn from(value: ToStrError) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<MangaDexError> for Error {
    fn from(value: MangaDexError) -> Self {
        Self::MangaDex(value)
    }
}

impl From<serde_json_path_to_error::Error> for Error {
    fn from(value: serde_json_path_to_error::Error) -> Self { 
        Self::custom(format!("{}: {value}", value.path()))
    }
}

pub(crate) trait ResponseToError<R> {
    fn manga_dex_response<T: DeserializeOwned + IntoData<R>>(self) -> impl Future<Output=Result<R, Error>>;
    fn manga_dex_template<S: DeserializeOwned + IntoData<R>>(self) -> impl Future<Output=Result<R, Error>>;
    fn manga_dex_response_empty(self) -> impl Future<Output=Result<(), Error>>;
}

impl<R> ResponseToError<R> for reqwest::Response {
    async fn manga_dex_response<T: DeserializeOwned + IntoData<R>>(self) -> Result<R, Error> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(Error::Authorization)
        } else if self.status().is_redirection() {
            Err(Error::http(self.status(), self.status().canonical_reason().unwrap_or(self.status().as_str())))
        } else {
            let response: MangaDexResponse<T> = self.json_with_error_path().await?;
            response.ok().map(|v| v.into_data())
        }
    }

    async fn manga_dex_response_empty(self) -> Result<(), Error> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(Error::Authorization)
        } else if self.status().is_redirection() {
            Err(Error::http(self.status(), self.status().canonical_reason().unwrap_or(self.status().as_str())))
        } else {
            let _ = self.bytes().await;
            Ok(())
        }
    }

    async fn manga_dex_template<S: DeserializeOwned + IntoData<R>>(self) -> Result<R, Error> {
        if self.status() == StatusCode::UNAUTHORIZED {
            Err(Error::Authorization)
        } else if !self.status().is_success() {
            let status = self.status();
            let body = self.text().await?;
            Err(Error::http(status, body))
        } else {
            Ok(self.json_with_error_path::<S>().await?.into_data())
        }
    }
}

#[macro_export]
macro_rules! bail {
    ($fmt: literal $(, $($arg: expr),* $(,)?)?) => {
        return Err(Error::custom(format!($fmt $(, $($arg,)*)?)))
    }
}
