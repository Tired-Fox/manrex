use std::{borrow::Cow, collections::BTreeMap};

use chrono::{DateTime, Duration, Local, TimeZone};
use reqwest::{header::HeaderMap, Response, StatusCode};

use crate::Error;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rate {
    /// X-RateLimit-Limit
    limit: usize,
    /// X-RateLimit-Remaining
    remaining: usize,
    /// X-RateLimit-Retry-After: unix timestamp
    retry_after: DateTime<Local>,
}

impl Rate {
    pub fn new(limit: usize, remaining: usize, retry_after: DateTime<Local>) -> Self {
        Self {
            limit,
            remaining,
            retry_after
        }
    }

    pub fn from_headers(headers: &HeaderMap) -> Result<Option<Self>, Error> {
        if headers.contains_key("X-RateLimit-Limit") {
            let limit = headers.get("X-RateLimit-Limit").unwrap().to_str()?.parse::<usize>().map_err(Error::custom)?;
            let remaining = headers.get("X-RateLimit-Remaining").unwrap().to_str()?.parse::<usize>().map_err(Error::custom)?;
            let retry_after = headers.get("X-RateLimit-Retry-After").unwrap().to_str()?.parse::<i64>().map_err(Error::custom)?;

            Ok(Some(Self::new(limit, remaining, Local.timestamp_opt(retry_after, 0).latest().unwrap_or_else(Local::now))))
        } else {
            Ok(None)
        }
    }

    pub fn limited(&self) -> bool {
        self.remaining == 0
            && self.retry_after > Local::now()
    }
}

/// Per Endpoint Rate Limiting
#[derive(Debug)]
pub struct RateLimiter {
    general: Rate,
    limits: BTreeMap<Cow<'static, str>, Rate>
}
impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            general: Rate::new(5, 5, Local::now()),
            limits: Default::default(),
        }
    }
}

impl RateLimiter {
    pub fn request(&mut self, endpoint: impl AsRef<str>) -> Result<(), Error> {
        if let Some(rate_limit) = self.limits.get(endpoint.as_ref()) {
            if rate_limit.limited() {
                return Err(Error::http(StatusCode::TOO_MANY_REQUESTS, format!(
                    "'{}' request limit reached. Wait until {} and try again",
                    endpoint.as_ref(),
                    rate_limit.retry_after,
                )));
            }
        }

        if self.general.limited() {
            return Err(Error::http(StatusCode::TOO_MANY_REQUESTS, "general http request limit reached. Wait 1 second and try again"))
        } else {
            if Local::now() - self.general.retry_after  > Duration::seconds(1) {
                self.general.remaining = 5;
            }

            self.general.remaining = self.general.remaining.saturating_sub(1);
            if self.general.remaining == 0 {
                self.general.retry_after = Local::now() + Duration::seconds(1);
            } else {
                self.general.retry_after = Local::now();
            }
        }

        Ok(())
    }

    pub fn update(&mut self, endpoint: impl std::fmt::Display, response: &Response) -> Result<(), Error> {
        if let Some(rate) = Rate::from_headers(response.headers())? {
            let endpoint = endpoint.to_string();
            self.limits.insert(endpoint.clone().into(), rate.clone());
            if rate.limited() {
                return Err(Error::http(StatusCode::TOO_MANY_REQUESTS, format!(
                    "'{endpoint}' request limit reached. Wait until {} and try again",
                    rate.retry_after,
                )));
            }
        }
        Ok(())
    }
}
