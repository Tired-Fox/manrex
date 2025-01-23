/*
    Client will have the ability to have middleware to be added. This middleware
    will be able to interact with state and manipulate the request before it is send.

    The middleware will either pass to the next middleware or response with a response.
*/

pub(crate) mod request;
pub mod auth;

use crate::{error::{DataStream, ResponseToError}, model::{at_home::AtHome, author::{Author, AuthorFilter, AuthorInclude, CreateAuthor, UpdateAuthor}, chapter::{Chapter, ChapterFilter, UpdateChapter}, client::{ApiClient, ClientFilter, ClientInclude}, cover::{Cover, CoverArtFilter, CoverInclude, CoverSize, EditCover, UploadCover}, manga::{Manga, MangaFilter}, Data, Paginated}};

use auth::OAuth;
use bytes::Bytes;
use chrono::{DateTime, Duration, Local};
use futures_util::Stream;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::{json, Value};

use crate::Error;
pub use request::{Request, ExtendParams};

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
    Uploads,
}
impl std::fmt::Display for MangaDex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api => write!(f, "https://api.mangadex.org"),
            Self::DevApi => write!(f, "https://api.mangadex.dev"),
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
    Cover,
    Manga,
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
            Self::Cover => write!(f, "cover"),
            Self::Manga => write!(f, "manga"),
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

pub struct IntoOptionalConcrete;
impl<A: Into<T>, T> Optional<T, IntoOptionalConcrete> for A {
    fn optional(self) -> Option<T> {
        Some(self.into())
    }
}

pub struct Client {
    pub(crate) oauth: OAuth,
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
}

/*
* -----[ AUTHORIZED CLIENTS ]-----
*/

// ---[ Client Endpoints ]---
impl Client {
    /// Get a list of clients based on the provided filters
    pub async fn get_clients<M>(
        &mut self,
        filters: impl Optional<ClientFilter, M>,
    ) -> Result<Paginated<Vec<ApiClient>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<ApiClient>>>().await
    }

    /// Create a new personal client
    pub async fn create_client<M>(&mut self, name: impl std::fmt::Display, description: impl Optional<String, M>) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let mut body = json!({
            "name": name.to_string(),
            "profile": "personal",
            "version": 1
        });

        if let Some(description) = description.optional() {
            body
                .as_object_mut()
                .unwrap()
                .insert("description".into(), serde_json::Value::String(description));
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&body)
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }
    
    /// Delete a client
    pub async fn delete_client(&mut self, id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    /// Edit a client's version and description
    pub async fn edit_client(&mut self, id: impl std::fmt::Display, version: usize, description: impl std::fmt::Display) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "description": description.to_string(),
                "version": version,
            }))
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }

    /// Get a client by it's id
    pub async fn get_client_by_id<M>(&mut self, id: impl std::fmt::Display, includes: impl Optional<Vec<ClientInclude>, M>) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }

    /// Get a client's secret
    pub async fn get_secret_by_client_id(&mut self, id: impl std::fmt::Display) -> Result<String, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .join("secret")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Data<String>>().await
    }

    /// Regenerate a clients secret
    pub async fn regenerate_client_secret(&mut self, id: impl std::fmt::Display) -> Result<String, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .join("secret")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({}))
            .send()
            .await?;

        res.manga_dex_response::<Data<String>>().await
    }
}

// ---[ AtHome Endpoints ]---
impl Client {
    pub async fn get_at_home_server(&mut self, chapter: impl std::fmt::Display, force_port: bool) -> Result<AtHome, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::AtHome))
            .join(chapter.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("forcePort443", force_port.then_some(true))
            .send()
            .await?;

        res.manga_dex_response::<AtHome>().await
    }
}

// ---[ Author Endpoints ]---
impl Client {
    pub async fn list_authors<M>(&mut self, filters: impl Optional<AuthorFilter, M>) -> Result<Paginated<Vec<Author>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Author))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Author>>>().await
    }

    pub async fn create_author(&mut self, author: CreateAuthor) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Author))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&author)
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn get_author<M>(&mut self, id: impl std::fmt::Display, includes: impl Optional<Vec<AuthorInclude>, M>) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Author))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn update_author(&mut self, id: impl std::fmt::Display, author: UpdateAuthor) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Author))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&author)
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn delete_author(&mut self, id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Author))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }
}

// ---[ Chapter Endpoints ]---
impl Client {
    /// Can use this endpoint to solve captchas explicitly.
    ///
    /// Otherwise adding `X-Captcha-Result` to the client headers and when it is sent with a request
    /// it will be verified, this will save 1 request call.
    pub async fn solve_captcha(&mut self, challenge: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Captcha))
            .join("solve")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&Value::String(challenge.to_string()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }
}

// ---[ Chapter Endpoints ]---
impl Client {
    pub async fn list_chapters<M>(&mut self, filters: impl Optional<ChapterFilter, M>) -> Result<Paginated<Vec<Chapter>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Chapter))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Chapter>>>().await
    }

    pub async fn get_chapter(&mut self, id: impl std::fmt::Display) -> Result<Chapter, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Chapter))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Data<Chapter>>().await
    }

    pub async fn update_chapter(
        &mut self,
        id: impl std::fmt::Display,
        chapter: UpdateChapter,
    ) -> Result<Chapter, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Chapter))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&chapter)
            .send()
            .await?;

        res.manga_dex_response::<Data<Chapter>>().await
    }

    pub async fn delete_chapter(
        &mut self,
        id: impl std::fmt::Display,
    ) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Chapter))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }
}

// ---[ Cover Endpoints ]---
impl Client {
    pub async fn list_covers<M>(&mut self, filter: impl Optional<CoverArtFilter, M>) -> Result<Paginated<Vec<Cover>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Cover))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Cover>>>().await
    }

    pub async fn upload_cover(&mut self, id: impl std::fmt::Display, cover: UploadCover) -> Result<Cover, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Cover))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .multipart(cover.into())
            .send()
            .await?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn get_cover<M>(&mut self, id: impl std::fmt::Display, includes: impl Optional<Vec<CoverInclude>, M>) -> Result<Cover, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Cover))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn retrieve_cover<M>(
        &mut self,
        manga_id: impl std::fmt::Display,
        file_name: impl std::fmt::Display,
        size: impl Optional<CoverSize, M>
    ) -> Result<DataStream, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let file_name = if let Some(size) = size.optional() {
            format!("{file_name}.{size}.jpg")
        } else {
            file_name.to_string()
        };

        let res = Request::get((MangaDex::Uploads, "covers"))
            .join(manga_id.to_string())
            .join(file_name)
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .send()
            .await?;

        ResponseToError::<()>::stream(res)
    }

    pub async fn edit_cover(&mut self, id: impl std::fmt::Display, cover: EditCover) -> Result<Cover, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Cover))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&cover)
            .send()
        .await?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn delete_cover(&mut self, id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Cover))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
        .await?;

        res.manga_dex_response::<()>().await
    }
}

// ---[ Manga Endpoints ]---
impl Client {
    pub async fn list_manga<M>(&mut self, filter: impl Optional<MangaFilter, M>) -> Result<Paginated<Vec<Manga>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filter.optional())
            .send()
        .await?;

        res.manga_dex_response::<Paginated<Vec<Manga>>>().await
    }
}
