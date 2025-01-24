use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::statistics::*,
    Client, Error,
};

// ---[ Statistic Endpoints ]---
impl Client {
    pub async fn get_chapter_statistics(&mut self, id: impl std::fmt::Display) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("chapter")
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }

    pub async fn get_chapters_statistics<S: std::fmt::Display>(&mut self, chapters: impl IntoIterator<Item=S>) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("chapter")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param("chapter", chapters.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }

    pub async fn get_scanlation_group_statistics(&mut self, id: impl std::fmt::Display) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("group")
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }

    pub async fn get_scanlation_groups_statistics<S: std::fmt::Display>(&mut self, groups: impl IntoIterator<Item=S>) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("group")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param("group", groups.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }

    pub async fn get_manga_statistics(&mut self, id: impl std::fmt::Display) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("manga")
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }

    pub async fn find_manga_statistics<S: std::fmt::Display>(&mut self, manga: impl IntoIterator<Item=S>) -> Result<BTreeMap<String, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("manga")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param("manga", manga.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<String, StatisticComments>>>().await
    }
}
