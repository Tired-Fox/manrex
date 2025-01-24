use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::{json, Value};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{rating::*, Data},
    Client, Error,
};

// ---[ Read Marker Endpoints ]---
impl Client {
    pub async fn list_read_markers(
        &mut self,
        manga: impl std::fmt::Display,
    ) -> Result<Vec<String>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(manga.to_string())
            .join("read")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param("manga", manga.to_string())
            .send()
            .await?;

        res.manga_dex_response::<Data<Vec<String>>>().await
    }

    pub async fn set_read_markers<M1, M2>(
        &mut self,
        manga: impl std::fmt::Display,
        chapters_read: Vec<String>,
        update_history: impl Optional<bool, M1>,
        chapter_ids_unread: impl Optional<Vec<String>, M1>,
    ) -> Result<Vec<String>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let mut body = json!({
            "chapterIdsRead": chapters_read,
        });

        if let Some(unread) = chapter_ids_unread.optional() {
            body.as_object_mut().unwrap().insert(
                "chapterIdsUnread".into(),
                Value::Array(unread.into_iter().map(Value::String).collect()),
            );
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(manga.to_string())
            .join("read")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("updateHistory", update_history.optional())
            .json(&body)
            .send()
            .await?;

        res.manga_dex_response::<Data<Vec<String>>>().await
    }

    pub async fn check_read_markers<S: std::fmt::Display, M>(
        &mut self,
        ids: Vec<S>,
    ) -> Result<Vec<String>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join("read")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param(
                "ids",
                ids.into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<Vec<String>>>().await
    }

    pub async fn check_read_markers_grouped<S: std::fmt::Display, M>(
        &mut self,
        ids: Vec<S>,
    ) -> Result<BTreeMap<String, Vec<String>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("read")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param(
                "ids",
                ids.into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
            )
            .param("grouped", true)
            .send()
            .await?;

        res.manga_dex_response::<Data<BTreeMap<String, Vec<String>>>>()
            .await
    }

    pub async fn get_read_history<S: std::fmt::Display, M>(
        &mut self,
    ) -> Result<Vec<History>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .join("history")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Ratings<Vec<History>>>().await
    }
}
