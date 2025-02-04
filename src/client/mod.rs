/*
    Client will have the ability to have middleware to be added. This middleware
    will be able to interact with state and manipulate the request before it is send.

    The middleware will either pass to the next middleware or response with a response.
*/

pub(crate) mod request;
pub mod auth;
mod endpoints;
mod rate_limit;

use crate::{error::ResponseToError, model::{at_home::{AtHome, AtHomeImageReport}, forum::{Resource, Thread}, Data}, Uuid};

use auth::OAuth;
use rate_limit::RateLimiter;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;

use crate::Error;
pub use request::{Request, ExtendParams};

pub static CLIENT_NAME: &str = std::env!("CARGO_PKG_NAME");
pub static CLIENT_VERSION: &str = std::env!("CARGO_PKG_VERSION");

pub enum MangaDex {
    Api,
    ApiDev,
    ApiNetwork,
    Auth,
    Uploads,
}
impl std::fmt::Display for MangaDex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api => write!(f, "https://api.mangadex.org"),
            Self::ApiDev => write!(f, "https://api.mangadex.dev"),
            Self::ApiNetwork => write!(f, "https://api.mangadex.network"),
            Self::Auth => write!(f, "https://auth.mangadex.org"),
            Self::Uploads => write!(f, "https://uploads.mangadex.org"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Endpoint {
    Ping,
    Client,
    AtHome,
    Chapter,
    Author,
    Captcha,
    Forums,
    Cover,
    Manga,
    Rating,
    User,
    Upload,
    Report,
    List,
    Group,
    Settings,
    Statistics,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "ping"),
            Self::Client => write!(f, "client"),
            Self::AtHome => write!(f, "at-home/server"),
            Self::Chapter => write!(f, "chapter"),
            Self::Author => write!(f, "author"),
            Self::Captcha => write!(f, "captcha"),
            Self::Forums => write!(f, "forums"),
            Self::Cover => write!(f, "cover"),
            Self::Manga => write!(f, "manga"),
            Self::Rating => write!(f, "rating"),
            Self::User => write!(f, "user"),
            Self::Report => write!(f, "report"),
            Self::Group => write!(f, "group"),
            Self::Settings => write!(f, "settings"),
            Self::Statistics => write!(f, "statistics"),
            Self::Upload => write!(f, "upload"),
            Self::List => write!(f, "list"),
        }
    }
}

/// Allows for any type that implements `Into` for the inner type to be automatically cast.
///
/// This will allow for paramters that can be `None` or an unwrapped `Some` value.
///
/// # Example
///
/// ```
/// use manrex::client::Optional;
///
/// fn test<O: Optional<String, M>, M>(name: O) {
///     match name.optional() {
///         Some(value) => /* String */,
///         None => /* No Value */
///     }
/// }
///
/// test(None);
/// test("ManRex");
/// test(String::from("ManRex"));
/// /* ... and anything else that implements `Into` for `String` */
/// ```
pub trait Optional<T, M=()> {
    fn optional(self) -> Option<T>;
}

impl<T> Optional<T> for Option<T> {
    fn optional(self) -> Option<T> {
        self
    }
}

pub struct IntoOptionalFrom;
impl<A: Into<T>, T> Optional<T, IntoOptionalFrom> for A {
    fn optional(self) -> Option<T> {
        Some(self.into())
    }
}

pub struct Client {
    pub(crate) oauth: OAuth,
    rate_limit: RateLimiter,
    //at_home_cache: BTreeMap<String, Cache<Chapter>>
}

impl Client {
    pub fn new(oauth: OAuth) -> Self {
        Self {
            oauth,
            rate_limit: RateLimiter::default(),
        }
    }

    pub fn oauth(&self) -> &OAuth {
        &self.oauth
    }

    pub fn oauth_mut(&mut self) -> &mut OAuth {
        &mut self.oauth
    }
}

impl Client {
    pub async fn ping(&self) -> Result<(), Error> {
        Request::get((MangaDex::Api, Endpoint::Ping))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(ACCEPT, "text/plain")
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn get_at_home_server(&mut self, chapter: impl std::fmt::Display, force_port: bool) -> Result<AtHome, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        self.rate_limit.request("get_at_home_server")?;

        let res = Request::get((MangaDex::Api, Endpoint::AtHome))
            .join(chapter.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("forcePort443", force_port.then_some(true))
            .send()
            .await?;

        self.rate_limit.update("get_at_home_server", &res)?;

        res.manga_dex_response::<AtHome>().await
    }

    pub async fn at_home_image_report(&self, report: AtHomeImageReport) -> Result<(), Error> {
        let res = Request::post((MangaDex::ApiNetwork, Endpoint::Report))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .json(&report)
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    /// Can use this endpoint to solve captchas explicitly.
    ///
    /// Otherwise adding `X-Captcha-Result` to the client headers and when it is sent with a request
    /// it will be verified, this will save 1 request call.
    pub async fn solve_captcha(&mut self, challenge: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        self.rate_limit.request("solve_captcha")?;
        let res = Request::post((MangaDex::Api, Endpoint::Captcha))
            .join("solve")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&Value::String(challenge.to_string()))
            .send()
            .await?;
        self.rate_limit.update("solve_captcha", &res)?;

        res.manga_dex_response::<()>().await
    }


    pub async fn create_forum_thread(&mut self, id: impl Into<Uuid>, typ: Resource) -> Result<Thread, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        self.rate_limit.request("create_forum_thread")?;
        let res = Request::post((MangaDex::Api, Endpoint::Forums))
            .join("thread")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&serde_json::json!({
                "type": typ,
                "id": id.into(),
            }))
            .send()
            .await?;
        self.rate_limit.update("create_forum_thread", &res)?;

        res.manga_dex_response::<Data<Thread>>().await
    }
}
