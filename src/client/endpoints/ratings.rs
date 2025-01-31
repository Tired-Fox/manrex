use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::rating::{Rating, *},
    uuid::MangaId,
    Client, Error,
};

// ---[ Rating Endpoints ]---
impl Client {
    pub async fn get_your_ratings(
        &mut self,
        manga: impl Into<MangaId>,
    ) -> Result<BTreeMap<String, Rating>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Rating))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param("manga", manga.into().as_ref())
            .send()
            .await?;

        res.manga_dex_response::<Ratings<BTreeMap<String, Rating>>>()
            .await
    }

    pub async fn create_or_update_rating(
        &mut self,
        id: impl Into<MangaId>,
        rating: usize,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Rating))
            .join(id.into().as_ref())
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

    pub async fn delete_rating(&mut self, id: impl Into<MangaId>) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Rating))
            .join(id.into().as_ref())
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
