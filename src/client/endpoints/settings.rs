use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::settings::*,
    Client, Error,
};

// ---[ Settings Endpoints ]---
impl Client {
    pub async fn get_latest_settings_template<S: DeserializeOwned>(&mut self) -> Result<S, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Settings))
            .join("template")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_template::<S>().await
    }

    pub async fn create_settings_template<S: Serialize + DeserializeOwned>(&mut self, template: &S) -> Result<S, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Settings))
            .join("template")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(template)
            .send()
            .await?;

        res.manga_dex_template::<S>().await
    }

    pub async fn get_settings_template_by_version<S: DeserializeOwned>(&mut self, version: impl std::fmt::Display) -> Result<S, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Settings))
            .join("template")
            .join(version.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_template::<S>().await
    }

    pub async fn get_settings<S: DeserializeOwned>(&mut self, version: impl std::fmt::Display) -> Result<Settings<S>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Settings))
            .join("template")
            .join(version.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Settings<S>>().await
    }

    pub async fn create_or_update_settings<S: Serialize + DeserializeOwned>(&mut self, settings: &S) -> Result<Settings<S>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Settings))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(settings)
            .send()
            .await?;

        res.manga_dex_response::<Settings<S>>().await
    }
}
