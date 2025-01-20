/*
    Client will have the ability to have middleware to be added. This middleware
    will be able to interact with state and manipulate the request before it is send.

    The middleware will either pass to the next middleware or response with a response.
*/

mod request;
pub mod auth;

use crate::model::{client::{ApiClient, ClientFilter}, MangaDexResponse, Paginated};

use auth::OAuth;
use chrono::{DateTime, Duration, Local};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::Error;
pub use request::{Request, ExtendParams, IntoUri};

pub static CLIENT_NAME: &str = std::env!("CARGO_PKG_NAME");
pub static CLIENT_VERSION: &str = std::env!("CARGO_PKG_VERSION");

//#[derive(Default, Debug, Clone, Copy, PartialEq)]
//pub struct Rate {
//    /// X-RateLimit-Limit
//    limit: usize,
//    /// X-RateLimit-Remaining
//    remaining: usize,
//    /// X-RateLimit-Retry-After: unix timestamp
//    retry_after: chrono::DateTime<chrono::Local>,
//}
//
///// Per Endpoint Rate Limiting
//#[derive(Debug, Default)]
//pub struct RateLimiter {
//    limits: BTreeMap<Endpoint, Rate>
//}
//
//impl RateLimiter {
//    pub fn request(&mut self, endpoint: Endpoint) -> Result<(), Error> {
//        match self.limits.get(&endpoint) {
//            Some(rate) if rate.remaining.saturating_sub(1) == 0 && chrono::Local::now() < rate.retry_after => {
//                return Err(Error::RateLimit)
//            },
//            _ => {}
//        }
//        Ok(())
//    }
//
//    pub fn update(&mut self, endpoint: Endpoint, rate: Option<Rate>) {
//        match rate {
//            Some(rate) => { self.limits.insert(endpoint, rate); },
//            None => if self.limits.contains_key(&endpoint) {
//                self.limits.remove(&endpoint);
//            },
//        }
//    }
//}

pub enum MangaDex {
    Api,
    DevApi,
    Auth,
}
impl std::fmt::Display for MangaDex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api => write!(f, "https://api.mangadex.org"),
            Self::DevApi => write!(f, "https://api.mangadex.dev"),
            Self::Auth => write!(f, "https://auth.mangadex.org"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Endpoint {
    Ping,
    Client,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "ping"),
            Self::Client => write!(f, "client"),
        }
    }
}

pub struct Cache<T: Clone> {
    expires: DateTime<Local>,
    data: T
}

impl<T: Clone> Cache<T> {
    pub fn new(data: T, duration: Duration) -> Self {
        Self {
            expires: Local::now() + duration,
            data
        }
    }
}

pub trait Optional<T> {
    fn optional(self) -> T;
}

impl<T: Default> Optional<T> for Option<T> {
    fn optional(self) -> T {
        match self {
            Some(value) => value,
            None => T::default()
        }
    }
}

impl<T> Optional<T> for T {
    fn optional(self) -> T {
        self
    }
}

pub struct Client {
    oauth: OAuth,
    //rate_limit: RateLimiter,
    //at_home_cache: BTreeMap<String, Cache<Chapter>>
}

impl Client {
    pub fn new(oauth: OAuth) -> Self {
        Self {
            oauth
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

    /*
    * -----[ AUTHORIZED CLIENTS ]-----
    */

    pub async fn get_clients(
        &mut self,
        filters: impl Optional<ClientFilter>,
    ) -> Result<Paginated<Vec<ApiClient>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params(filters.optional())
            .send()
            .await?;

        let body: MangaDexResponse<Paginated<Vec<ApiClient>>> = res.json().await?;
        body.ok()
    }

    pub async fn create_client(&mut self, name: impl std::fmt::Display, description: Option<String>, version: usize) -> Result<Paginated<Vec<ApiClient>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let mut body = json!({
            "name": name.to_string(),
            "profile": "personal",
            "version": version,
        });

        if let Some(description) = description {
            body
                .as_object_mut()
                .unwrap()
                .insert("description".into(), serde_json::Value::String(description));
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&body)?
            .send()
            .await?;

        let body: MangaDexResponse<Paginated<Vec<ApiClient>>> = res.json().await?;
        body.ok()
    }

    ///// `/manga/random`
    //async fn get_random_manga(&mut self) -> Result<Manga, Error> {
    //    self.rate_limit.request(Endpoint::GetRandomManga)?;
    //
    //    // TODO: Fetch Data...
    //    // TODO: Update rate limit
    //
    //    Ok(Manga {})
    //}

    //async fn get_chapter(&mut self, id: impl std::fmt::Display) -> Result<Chapter, Error> {
    //    let id = id.to_string();
    //    match self.at_home_cache.get(&id) {
    //        Some(at_home) if at_home.expires > Local::now() => {
    //            return Ok(at_home.data.clone());
    //        },
    //        _ => {}
    //    }
    //    self.rate_limit.request(Endpoint::GetChapter)?;
    //
    //    // TODO: Fetch Data
    //    let result = Chapter {};
    //
    //    // TODO: Update rate limit
    //    self.rate_limit.update(Endpoint::GetChapter, None);
    //    self.at_home_cache.insert(id, Cache::new(result.clone(), Duration::minutes(15)));
    //
    //    Ok(result)
    //}
}
