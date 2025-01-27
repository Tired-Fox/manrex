use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
};

use crate::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    multipart, Body, Client, Method,
};

#[derive(Debug, Clone, PartialEq)]
pub enum OneOrMany<S> {
    One(S),
    Many(Vec<S>),
}
impl<S> From<S> for OneOrMany<S> {
    fn from(value: S) -> Self {
        Self::One(value)
    }
}
impl<S> From<Vec<S>> for OneOrMany<S> {
    fn from(value: Vec<S>) -> Self {
        Self::Many(value)
    }
}
impl<S: std::fmt::Display> From<OneOrMany<S>> for Param {
    fn from(value: OneOrMany<S>) -> Self {
        match value {
            OneOrMany::One(value) => Param::Value(value.to_string()),
            OneOrMany::Many(value) => {
                Param::Array(value.into_iter().map(|v| v.to_string()).collect())
            }
        }
    }
}

#[derive(Debug, Clone, strum::EnumIs, PartialEq)]
pub enum Param {
    Array(Vec<String>),
    Map(BTreeMap<String, String>),
    Value(String),
}

macro_rules! into_param {
    ($($typ: ty),*) => {
        $(
            impl From<$typ> for Param {
                fn from(value: $typ) -> Self {
                    Self::Value(value.to_string())
                }
            }
        )*
    }
}

into_param! {
    String, &str,
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    bool
}

impl<'a> From<std::borrow::Cow<'a, str>> for Param {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        Self::Value(value.to_string())
    }
}

impl<S: std::fmt::Display> From<Vec<S>> for Param {
    fn from(value: Vec<S>) -> Self {
        Self::Array(value.into_iter().map(|v| v.to_string()).collect())
    }
}
impl<S: std::fmt::Display> From<HashSet<S>> for Param {
    fn from(value: HashSet<S>) -> Self {
        Self::Array(value.into_iter().map(|v| v.to_string()).collect())
    }
}
impl<S: std::fmt::Display> From<&[S]> for Param {
    fn from(value: &[S]) -> Self {
        Self::Array(value.iter().map(|v| v.to_string()).collect())
    }
}
impl<S: std::fmt::Display, V: std::fmt::Display> From<BTreeMap<S, V>> for Param {
    fn from(value: BTreeMap<S, V>) -> Self {
        Self::Map(
            value
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }
}

impl Param {
    fn encode(&self, name: &str) -> String {
        match self {
            Self::Array(values) => {
                url::form_urlencoded::byte_serialize(values.join(",").as_bytes())
                    .collect::<String>()
            }
            Self::Value(value) => {
                url::form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>()
            }
            Self::Map(values) => values
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}={}",
                        url::form_urlencoded::byte_serialize(format!("{name}[{k}]").as_bytes())
                            .collect::<String>(),
                        url::form_urlencoded::byte_serialize(v.as_bytes()).collect::<String>(),
                    )
                })
                .collect::<Vec<_>>()
                .join("&"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Params(BTreeMap<String, Param>);
impl Params {
    pub fn insert(&mut self, key: impl std::fmt::Display, value: impl Into<Param>) {
        self.0.insert(key.to_string(), value.into());
    }

    #[allow(dead_code)]
    pub fn extend<K: std::fmt::Display, P: Into<Param>>(
        &mut self,
        params: impl IntoIterator<Item = (K, P)>,
    ) {
        self.0
            .extend(params.into_iter().map(|(k, v)| (k.to_string(), v.into())));
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .0
            .iter()
            .map(|(key, param)| {
                if param.is_array() {
                    format!(
                        "{}={}",
                        url::form_urlencoded::byte_serialize(format!("{key}[]").as_bytes())
                            .collect::<String>(),
                        param.encode(key)
                    )
                } else if param.is_map() {
                    param.encode(key)
                } else {
                    format!("{key}={}", param.encode(key))
                }
            })
            .collect::<Vec<_>>()
            .join("&");

        write!(f, "{params}")
    }
}

pub struct Request {
    method: Method,
    uri: PathBuf,
    headers: HeaderMap,
    params: Params,
    body: Option<Body>,
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
    fn extend_params(self, request: &mut Request);
}

impl Request {
    pub fn new<M>(method: Method, uri: impl IntoUri<M>) -> Self {
        Self {
            method,
            uri: PathBuf::from(uri.into_uri()),
            headers: HeaderMap::default(),
            params: Params::default(),
            body: None,
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

    pub fn add_param(&mut self, key: impl std::fmt::Display, value: impl Into<Param>) -> &mut Self {
        self.params.insert(key, value);
        self
    }

    pub fn add_param_opt<P: Into<Param>>(
        &mut self,
        key: impl std::fmt::Display,
        value: Option<P>,
    ) -> &mut Self {
        if let Some(value) = value {
            self.params.insert(key, value.into());
        }
        self
    }

    pub fn param(mut self, key: impl std::fmt::Display, value: impl Into<Param>) -> Self {
        self.params.insert(key, value);
        self
    }

    pub fn param_opt<V: Into<Param>>(
        mut self,
        key: impl std::fmt::Display,
        value: Option<V>,
    ) -> Self {
        if let Some(value) = value {
            self.params.insert(key.to_string(), value);
        }
        self
    }

    #[allow(dead_code)]
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
        self.headers.insert(
            key,
            value
                .to_string()
                .parse::<HeaderValue>()
                .expect("failed to parse HeaderValue"),
        );
        self
    }

    pub fn json<S: serde::Serialize>(self, body: &S) -> reqwest::RequestBuilder {
        let url = if self.params.is_empty() {
            self.uri.display().to_string().replace("\\", "/")
        } else {
            format!(
                "{}?{}",
                self.uri.display().to_string().replace("\\", "/"),
                self.params
            )
        };

        Client::new()
            .request(self.method, url)
            .headers(self.headers)
            .json(body)
    }

    pub fn multipart(self, form: multipart::Form) -> reqwest::RequestBuilder {
        let url = if self.params.is_empty() {
            self.uri.display().to_string().replace("\\", "/")
        } else {
            format!(
                "{}?{}",
                self.uri.display().to_string().replace("\\", "/"),
                self.params
            )
        };

        Client::new()
            .request(self.method, url)
            .headers(self.headers)
            .multipart(form)
    }

    pub fn form<S: serde::Serialize>(self, body: &S) -> reqwest::RequestBuilder {
        let url = if self.params.is_empty() {
            self.uri.display().to_string().replace("\\", "/")
        } else {
            format!(
                "{}?{}",
                self.uri.display().to_string().replace("\\", "/"),
                self.params
            )
        };

        Client::new()
            .request(self.method, url)
            .headers(self.headers)
            .form(body)
    }

    pub async fn send(self) -> Result<reqwest::Response, Error> {
        let url = if self.params.is_empty() {
            self.uri.display().to_string().replace("\\", "/")
        } else {
            format!(
                "{}?{}",
                self.uri.display().to_string().replace("\\", "/"),
                self.params
            )
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
