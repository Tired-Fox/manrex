use std::{collections::BTreeMap, path::{Path, PathBuf}};

use crate::Error;
use reqwest::{header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE}, Body, Client, Method};

pub struct Request {
    method: Method,
    uri: PathBuf,
    headers: HeaderMap,
    params: BTreeMap<String, String>,
    body: Option<Body>
}

pub trait IntoUri<M = ()> {
    fn into_uri(self) -> String;
}

impl<T: reqwest::IntoUrl> IntoUri for T {
    fn into_uri(self) -> String {
        self.into_url().unwrap().to_string()
    }
}

pub struct PairToUri;
impl<A, B> IntoUri<PairToUri> for (A, B)
where
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    fn into_uri(self) -> String {
        format!("{}/{}", self.0, self.1)
    }
}

pub trait ExtendParams {
    fn extend_params(&self, request: &mut Request);
}

impl Request {
    pub fn new<M>(method: Method, uri: impl IntoUri<M>) -> Self {
        Self {
            method,
            uri: PathBuf::from(uri.into_uri()),
            headers: HeaderMap::default(),
            params: BTreeMap::default(),
            body: None
        }
    }

    pub fn get<M>(uri: impl IntoUri<M>) -> Self {
        Self::new(Method::GET, uri)
    }

    pub fn post<M>(uri: impl IntoUri<M>) -> Self {
        Self::new(Method::POST, uri)
    }

    pub fn put<M>(uri: impl IntoUri<M>) -> Self {
        Self::new(Method::PUT, uri)
    }

    pub fn delete<M>(uri: impl IntoUri<M>) -> Self {
        Self::new(Method::DELETE, uri)
    }

    pub fn join(mut self, path: impl AsRef<Path>) -> Self {
        self.uri = self.uri.join(path);
        self
    }

    pub fn add_param(&mut self, key: impl std::fmt::Display, value: impl std::fmt::Display) -> &mut Self {
        self.params.insert(key.to_string(), url::form_urlencoded::byte_serialize(value.to_string().as_bytes()).collect());
        self
    }

    pub fn param(mut self, key: impl std::fmt::Display, value: impl std::fmt::Display) -> Self {
        self.params.insert(key.to_string(), url::form_urlencoded::byte_serialize(value.to_string().as_bytes()).collect());
        self
    }

    pub fn params(mut self, params: impl ExtendParams) -> Self {
        params.extend_params(&mut self);
        self
    }

    pub fn params_opt<P: ExtendParams>(mut self, params: Option<P>) -> Self {
        if let Some(params) = params {
            params.extend_params(&mut self);
        }
        self
    }

    pub fn header(mut self, key: impl IntoHeaderName, value: impl std::fmt::Display) -> Self {
        self.headers.insert(key, value.to_string().parse::<HeaderValue>().expect("failed to parse HeaderValue"));
        self
    }

    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body.replace(body.into());
        self
    }

    pub fn json<S: serde::Serialize>(mut self, body: &S) -> Result<Self, Error> {
        match serde_json::to_string(&body) {
            Ok(body) => {
                self.body.replace(body.into());
                Ok(self.header(CONTENT_TYPE, "application/json"))
            },
            Err(err) => Err(Error::from(err))
        }
    }

    pub fn form<S: serde::Serialize>(mut self, body: &S) -> Result<Self, Error> {
        match serde_urlencoded::to_string(&body) {
            Ok(body) => {
                self.body.replace(body.into());
                Ok(self.header(CONTENT_TYPE, "application/x-www-form-urlencoded"))
            },
            Err(err) => Err(Error::from(err))
        }
    }

    pub async fn send(self) -> Result<reqwest::Response, Error> {
        let url = if self.params.is_empty() {
            self.uri.display().to_string().replace("\\", "/")
        } else {
            let params = self.params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");
            format!("{}?{params}", self.uri.display().to_string().replace("\\", "/"))
        };

        let mut req = Client::new()
            .request(self.method, url)
            .headers(self.headers);

        if let Some(body) = self.body {
            // req.form(form)
            req = req.body(body);
        }

        Ok(req.send().await?)
    }
}
