use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::{json, Value};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{rating::*, Data},
    uuid::{ChapterId, MangaId},
    Client, Error,
};

// ---[ Read Marker Endpoints ]---
impl Client {
    pub async fn list_read_markers(
        &mut self,
        manga: impl Into<MangaId>,
    ) -> Result<Vec<ChapterId>, Error> {
        let id = manga.into();

        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.as_ref())
            .join("read")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param("manga", id.as_ref())
            .send()
            .await?;

        res.manga_dex_response::<Data<Vec<ChapterId>>>().await
    }

    pub async fn set_read_markers<M1, M2>(
        &mut self,
        manga: impl Into<MangaId>,
        chapters_read: impl IntoIterator<Item = ChapterId>,
        update_history: impl Optional<bool, M1>,
        chapter_ids_unread: impl Optional<Vec<ChapterId>, M1>,
    ) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let mut body = json!({
            "chapterIdsRead": chapters_read.into_iter().collect::<Vec<_>>(),
        });

        if let Some(unread) = chapter_ids_unread.optional() {
            body.as_object_mut().unwrap().insert(
                "chapterIdsUnread".into(),
                Value::Array(
                    unread
                        .into_iter()
                        .map(|v| Value::String(v.to_string()))
                        .collect(),
                ),
            );
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(manga.into().as_ref())
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

        res.manga_dex_response::<()>().await
    }

    /// List the chapters that are marked as read for multiple manga
    pub async fn list_multiple_read_markers<S: Into<MangaId>, M>(
        &mut self,
        ids: Vec<S>,
    ) -> Result<Vec<ChapterId>, Error> {
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
                ids.into_iter()
                    .map(|v| v.into().to_string())
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<Vec<ChapterId>>>().await
    }

    pub async fn list_multiple_read_markers_grouped<S: Into<MangaId>, M>(
        &mut self,
        ids: Vec<S>,
    ) -> Result<BTreeMap<MangaId, Vec<ChapterId>>, Error> {
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
                ids.into_iter()
                    .map(|v| v.into().to_string())
                    .collect::<Vec<_>>(),
            )
            .param("grouped", true)
            .send()
            .await?;

        res.manga_dex_response::<Data<BTreeMap<MangaId, Vec<ChapterId>>>>()
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
