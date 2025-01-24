use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{rating::Rating, rating::*},
    Client, Error,
};

// ---[ Rating Endpoints ]---
impl Client {
    pub async fn get_your_ratings(
        &mut self,
        manga: impl std::fmt::Display,
    ) -> Result<BTreeMap<String, Rating>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Rating))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param("manga", manga.to_string())
            .send()
            .await?;

        res.manga_dex_response::<Ratings<BTreeMap<String, Rating>>>()
            .await
    }

    pub async fn create_or_update_rating(
        &mut self,
        id: impl std::fmt::Display,
        rating: usize,
    ) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Rating))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({
                "rating": rating.min(10)
            }))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn delete_rating(&mut self, id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Rating))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }
}
