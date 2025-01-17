#[derive(Debug)]
pub enum Error {
    /// HTTP 429 Rate Limit Reached
    RateLimit,
    UnknownHost,
    Custom(String),
    Io(std::io::Error),
    Http(hyper::http::Error),
    Hyper(hyper::Error),
}

impl std::error::Error for Error {}

impl Error {
    pub fn custom(msg: impl std::fmt::Display) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RateLimit => write!(f, "rate limit has been reached"),
            Self::UnknownHost => write!(f, "unknown host in http request"),
            Self::Custom(msg) => write!(f, "{msg}"),
            Self::Http(err) => write!(f, "{err}"),
            Self::Hyper(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Self::Http(value)
    }
}


impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::Hyper(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
