use std::{borrow::Cow, collections::BTreeMap, path::{Path, PathBuf}};

use chrono::{DateTime, Duration, Local, TimeZone};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{client::{MangaDex, Request}, Error};

use super::{CLIENT_NAME, CLIENT_VERSION};

fn deserialize_timestamp<'de, D: Deserializer<'de>>(de: D) -> Result<DateTime<Local>, D::Error> {
    let timestamp: i64 = serde::Deserialize::deserialize(de)?;
    Ok(Local.timestamp_opt(timestamp, 0).latest().expect("failed to parse unix timestamp"))
}

fn serialize_timestamp<S: serde::Serializer>(timestamp: &DateTime<Local>, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_i64(timestamp.timestamp())
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    access: Cow<'static, str>,
    refresh: Cow<'static, str>,
    #[serde(deserialize_with="deserialize_timestamp", serialize_with="serialize_timestamp")]
    expires: DateTime<Local>,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("access", &self.access)
            .field("expires", &self.expires)
            .finish_non_exhaustive()
    }
}

impl Token {
    pub fn refresh(&mut self, access: impl AsRef<str>) {
        self.access = access.as_ref().to_string().into();
        self.expires = Local::now() + Duration::minutes(15);
    }

    pub fn expired(&self) -> bool {
        self.expires - Local::now() <= Duration::minutes(15)
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Credentials {
    pub(crate) id: Cow<'static, str>,
    pub(crate) secret: Cow<'static, str>,
}

impl Credentials {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl Credentials {
    pub fn new(client_id: impl std::fmt::Display, client_secret: impl std::fmt::Display) -> Self {
        Self {
            id: client_id.to_string().into(),
            secret: client_secret.to_string().into(),
        }
    }

    #[cfg(feature="env")]
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv().map_err(Error::custom)?;

        Ok(Self::new(
            std::env::var("MANGADEX_CLIENT_ID")?,
            std::env::var("MANGADEX_CLIENT_SECRET")?
        ))
    }
}

#[derive(serde::Deserialize)]
struct AuthToken {
    access_token: String,
    refresh_token: String,
}
impl From<AuthToken> for Token {
    fn from(value: AuthToken) -> Self {
        Token {
            access: value.access_token.into(),
            refresh: value.refresh_token.into(),
            expires: Local::now() + Duration::minutes(15),
        }
    }
}

#[derive(serde::Deserialize)]
struct RefreshToken {
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct OAuth {
    pub(crate) cache_path: PathBuf,

    pub(crate) token: Option<Token>,
    pub(crate) credentials: Credentials,
}

impl OAuth {
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn set_credentials(&mut self, creds: Credentials) {
        self.credentials = creds;
    }

    pub fn new(creds: Credentials) -> Self {
        Self::new_with_cache(
            creds,
            dirs::cache_dir().expect("failed to get systems cache directory").join("manrex")
        )
    }

    pub fn new_with_cache(creds: Credentials, cache: impl AsRef<Path>) -> Self {
        let cache_path = cache.as_ref().to_path_buf();

        let token = if cache_path.join("token.json").exists() {
            match std::fs::read_to_string(cache_path.join("token.json")) {
                Ok(file) => serde_json::from_str(&file).ok(),
                Err(_) => None
            }
        } else {
            None
        };

        Self {
            cache_path,
            token,
            credentials: creds,
        }
    }

    pub fn access_token(&self) -> String {
        self.token.as_ref().expect("failed to get access_token").access.to_string()
    }

    pub fn expired(&self) -> Result<bool, Error> {
        match self.token.as_ref() {
            Some(token) => Ok(token.expired()),
            None => Err(Error::Authorization)
        }
    }

    pub fn logged_in(&self) -> bool {
        self.token.is_some()
    }

    pub fn logout(&self) -> Result<(), Error> {
        if self.cache_path.join("token.json").exists() {
            std::fs::remove_file(self.cache_path.join("token.json"))?;
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        if let Some(token) = self.token.as_ref() {
            if !self.cache_path.exists() {
                std::fs::create_dir_all(&self.cache_path)?;
            }
            std::fs::write(self.cache_path.join("token.json"), serde_json::to_string(token)?)?;
        }
        Ok(())
    }

    /// Login with the given username and password
    pub async fn login_with(&mut self, username: impl std::fmt::Display, password: impl std::fmt::Display) -> Result<(), Error> {
        let res = Request::post((MangaDex::Auth, "realms/mangadex/protocol/openid-connect/token"))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .form(&BTreeMap::from([
                ("grant_type", "password".to_string()),
                ("username", username.to_string()),
                ("password", password.to_string()),
                ("client_id", self.credentials.id.to_string()),
                ("client_secret", self.credentials.secret.to_string()),
            ]))
            .send()
            .await?
            .error_for_status()?;

        let token: AuthToken = res.json().await?;
        self.token.replace(token.into());

        self.save()
    }

    pub async fn refresh(&mut self) -> Result<(), Error> { 
        if let Some(token) = self.token.as_mut() {
            let res = Request::post((MangaDex::Auth, "realms/mangadex/protocol/openid-connect/token"))
                .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
                .form(&BTreeMap::from([
                    ("grant_type", "refresh_token".to_string()),
                    ("refresh_token", token.refresh.to_string()),
                    ("client_id", self.credentials.id.to_string()),
                    ("client_secret", self.credentials.secret.to_string()),
                ]))
                .send()
                .await?
                .error_for_status()?;


            let rt: RefreshToken = res.json().await?;
            token.refresh(rt.access_token);

            self.save()
        } else {
            Err(Error::Authorization)
        }
    }
}
