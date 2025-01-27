use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    client::{MangaDex, Request},
    de::deserialize_timestamp,
    ser::serialize_timestamp,
    Error,
};
use chrono::{DateTime, Duration, Local};
use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, header::CONTENT_TYPE, Method};

thread_local! {
    static TOKEN_PATH: PathBuf = dirs::cache_dir()
        .expect("failed to get cache directory")
        .join("manrex")
        .join("token.json")
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Token {
    pub(crate) token: String,
    pub(crate) refresh: String,
    #[serde(
        deserialize_with = "deserialize_timestamp",
        serialize_with = "serialize_timestamp"
    )]
    pub(crate) timestamp: DateTime<Local>,
}

impl Token {
    pub fn new(token: String, refresh: String) -> Self {
        Self {
            token,
            refresh,
            timestamp: Local::now(),
        }
    }

    /// Token
    pub fn value(&self) -> &str {
        &self.token
    }

    /// Refresh token
    pub fn refresh(&self) -> &str {
        &self.refresh
    }

    /// Check if the token is expired based on the timestamp of when it was fetched
    pub fn is_expired(&self) -> bool {
        (self.timestamp - Local::now()) >= Duration::minutes(15)
    }

    /// Manually expire the token
    pub fn expire(&mut self) {
        self.timestamp -= Duration::minutes(15);
    }
}

#[derive(Clone, PartialEq)]
pub struct Credentials {
    // Client ID
    id: String,
    // Client Secret
    secret: String,
}

impl Credentials {
    pub fn new(client_id: impl std::fmt::Display, client_secret: impl std::fmt::Display) -> Self {
        Self {
            id: client_id.to_string(),
            secret: client_secret.to_string(),
        }
    }

    #[cfg(feature = "env")]
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv()?;
        Ok(Self::new(
            std::env::var("MANREX_CLIENT_ID")?,
            std::env::var("MANREX_CLIENT_SECRET")?,
        ))
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("id", &self.id)
            .field("secret", &"[SECRET]")
            .finish()
    }
}

#[derive(Debug)]
pub struct PasswordAuth {
    credentials: Credentials,
}
impl PasswordAuth {
    pub async fn with_login(
        self,
        username: impl std::fmt::Display,
        password: impl std::fmt::Display,
    ) -> Result<OAuth, Error> {
        let args = BTreeMap::from([
            ("grant_type".to_string(), "password".to_string()),
            ("username".to_string(), username.to_string()),
            ("password".to_string(), password.to_string()),
            ("client_id".to_string(), self.credentials.id.clone()),
            ("client_secret".to_string(), self.credentials.secret.clone()),
        ]);

        let response = Request::new(
            Method::POST,
            MangaDex::Auth,
            "realms/mangadex/protocol/openid-connect/token",
        )
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Full::new(Bytes::from(serde_urlencoded::to_string(&args)?)))
        .send()
        .await?;

        println!("{}", response.status());
        println!("{:#?}", response.headers());
        let body = response.into_body().collect().await?.to_bytes().to_vec();
        let body = String::from_utf8_lossy(&body);

        //let variables: BTreeMap<String, String> = serde_json::from_str(&body)?;
        //let token = Token::new(
        //    variables
        //        .get("access_token")
        //        .expect("missing access_token in authorization request")
        //        .clone(),
        //    variables
        //        .get("refresh_token")
        //        .expect("missing refresh_token in authorization request")
        //        .clone(),
        //);

        println!("{body}");

        // TODO: Save Token

        Ok(OAuth {
            token: None,
            credentials: self.credentials,
        })
    }
}

#[derive(Debug, Clone)]
pub struct OAuth {
    token: Option<Token>,
    credentials: Credentials,
}

impl OAuth {
    /// Refresh the OAuth token
    pub fn refresh(&mut self) -> Result<(), Error> {
        todo!("implement token refresh")
    }

    pub fn new(credentials: Credentials) -> Result<Self, Error> {
        // TODO: Attempt to existing token file
        let token = match TOKEN_PATH.with(|path| {
            if path.exists() {
                let file = std::fs::read_to_string(path)?;
                Ok(Some(serde_json::from_str::<Token>(&file)?))
            } else {
                Ok(None)
            }
        }) {
            Ok(Some(token)) => token,
            Ok(None) => return Err(Error::LoginRequired(PasswordAuth { credentials })),
            Err(err) => return Err(err),
        };

        Ok(Self {
            token: Some(token),
            credentials,
        })
    }
}
