use reqwest::{header::ToStrError, StatusCode};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MangaDexError {
    id: String,
    status: usize,
    title: String,
    detail: Option<String>,
    context: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    /// HTTP 429 Rate Limit Reached
    RateLimit,
    Authorization,
    UnknownHost,

    Redirect(String),
    Http(StatusCode, String),
    MangaDex(MangaDexError),
    Group(Vec<Error>),

    Custom(String),
}

impl std::error::Error for Error {}

impl Error {
    pub fn custom(msg: impl std::fmt::Display) -> Self {
        Self::Custom(msg.to_string())
    }
    pub fn redirect(msg: impl std::fmt::Display) -> Self {
        Self::Redirect(msg.to_string())
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
            Self::RateLimit => write!(f, "rate limit has been reached"),
            Self::Authorization => write!(f, "attempt to call an authorized endpoint with an unauthorized client"),
            Self::UnknownHost => write!(f, "unknown host in http request"),
            Self::Custom(msg) => write!(f, "{msg}"),
            Self::Redirect(loc) => write!(f, "http 300 redirect to {loc}"),
            Self::Http(status, msg) => write!(f, "http [{}] {msg}", status.as_u16()),
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

pub trait ResponseToError {
    fn to_error(&self) -> Result<(), Error>;
}

impl ResponseToError for reqwest::Response {
    fn to_error(&self) -> Result<(), Error> {
        if self.status().is_client_error() || self.status().is_server_error() {
            Err(Error::http(self.status(), self.status().canonical_reason().unwrap_or(self.status().as_str())))
        } else if self.status().is_redirection() {
            Err(Error::redirect(self.headers().get("Location").unwrap().to_str()?))
        } else {
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! bail {
    ($fmt: literal $(, $($arg: expr),* $(,)?)?) => {
        return Err(Error::custom(format!($fmt $(, $($arg,)*)?)))
    }
}
