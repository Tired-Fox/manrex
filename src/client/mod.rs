/*
    Client will have the ability to have middleware to be added. This middleware
    will be able to interact with state and manipulate the request before it is send.

    The middleware will either pass to the next middleware or response with a response.
*/

use std::collections::BTreeMap;

use chrono::{DateTime, Duration, Local, TimeZone};
use http_body_util::Empty;
use hyper::{body::{Bytes, Incoming}, HeaderMap, Method, Response, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

use crate::Error;

// TODO: Create token structure
pub struct Token {}

// TODO: Implement credentials
pub struct Credentials {}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rate {
    /// X-RateLimit-Limit
    limit: usize,
    /// X-RateLimit-Remaining
    remaining: usize,
    /// X-RateLimit-Retry-After: unix timestamp
    retry_after: chrono::DateTime<chrono::Local>,
}

impl Rate {
    fn from(value: hyper::http::HeaderMap) -> Option<Self> {
        if value.contains_key("X-RateLimit-Limit") && value.contains_key("X-RateLimit-Remaining") && value.contains_key("X-RateLimit-Retry-After") {
            return Some(Self {
                limit: value.get("X-RateLimit-Limit")?.to_str().ok()?.parse().ok()?,
                remaining: value.get("X-RateLimit-Remaining")?.to_str().ok()?.parse().ok()?,
                retry_after: Local.timestamp_opt(value.get("X-RateLimit-Retry-After")?.to_str().ok()?.parse::<i64>().ok()?, 0).latest()?,
            })
        }
        None
    }
}

/// Per Endpoint Rate Limiting
#[derive(Debug, Default)]
pub struct RateLimiter {
    limits: BTreeMap<Endpoint, Rate>
}

impl RateLimiter {
    pub fn request(&mut self, endpoint: Endpoint) -> Result<(), Error> {
        match self.limits.get(&endpoint) {
            Some(rate) if rate.remaining.saturating_sub(1) == 0 && chrono::Local::now() < rate.retry_after => {
                return Err(Error::RateLimit)
            },
            _ => {}
        }
        Ok(())
    }

    pub fn update(&mut self, endpoint: Endpoint, rate: Option<Rate>) {
        match rate {
            Some(rate) => { self.limits.insert(endpoint, rate); },
            None => if self.limits.contains_key(&endpoint) {
                self.limits.remove(&endpoint);
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Endpoint {
    GetRandomManga,
    GetChapter,
    Ping,
}

pub struct Manga {}
#[derive(Clone)]
pub struct Chapter {}

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

pub struct Client {
    token: Token,
    credentials: Credentials,

    rate_limit: RateLimiter,

    at_home_cache: BTreeMap<String, Cache<Chapter>>
}

impl Client {
    /// `/manga/random`
    async fn get_random_manga(&mut self) -> Result<Manga, Error> {
        self.rate_limit.request(Endpoint::GetRandomManga)?;

        // TODO: Fetch Data...
        // TODO: Update rate limit

        Ok(Manga {})
    }

    async fn get_chapter(&mut self, id: impl std::fmt::Display) -> Result<Chapter, Error> {
        let id = id.to_string();
        match self.at_home_cache.get(&id) {
            Some(at_home) if at_home.expires > Local::now() => {
                return Ok(at_home.data.clone());
            },
            _ => {}
        }
        self.rate_limit.request(Endpoint::GetChapter)?;

        // TODO: Fetch Data
        let result = Chapter {};

        // TODO: Update rate limit
        self.rate_limit.update(Endpoint::GetChapter, None);
        self.at_home_cache.insert(id, Cache::new(result.clone(), Duration::minutes(15)));

        Ok(result)
    }
}

pub struct Request<B = Empty<Bytes>> {
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    body: B
}

impl<B: hyper::body::Body + Send + 'static> Request<B>
where
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>
{
    pub async fn send<R>(self) -> Result<Response<Incoming>, Error> {
        let host = self.uri.host().ok_or(Error::UnknownHost)?;
        let port = self.uri.port_u16().unwrap_or(80);

        let address = format!("{host}:{port}");

        let stream = TcpStream::connect(address).await?;

        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        // Connect
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection failed: {:?}", err);
            }
        });

        let authority = self.uri.authority().unwrap().clone();

        let mut req = hyper::Request::builder()
            .uri(self.uri)
            .header(hyper::header::HOST, authority.as_str())
            .header(hyper::header::USER_AGENT, "localhost")
            .method(self.method);
        req.headers_mut().map(|h| h.extend(self.headers));
        Ok(sender.send_request(req.body(self.body)?).await?)
    }
}
