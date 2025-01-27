use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::statistics::*,
    uuid::{ChapterId, GroupId, MangaId},
    Client, Error,
};

// ---[ Statistic Endpoints ]---
impl Client {
    pub async fn get_chapter_statistics(
        &mut self,
        id: impl Into<ChapterId>,
    ) -> Result<BTreeMap<ChapterId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("chapter")
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<ChapterId, StatisticComments>>>()
            .await
    }

    pub async fn get_chapters_statistics<S: Into<ChapterId>>(
        &mut self,
        chapters: impl IntoIterator<Item = S>,
    ) -> Result<BTreeMap<ChapterId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("chapter")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param(
                "chapter",
                chapters.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<ChapterId, StatisticComments>>>()
            .await
    }

    pub async fn get_scanlation_group_statistics(
        &mut self,
        id: impl Into<GroupId>,
    ) -> Result<BTreeMap<GroupId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("group")
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<GroupId, StatisticComments>>>()
            .await
    }

    pub async fn get_scanlation_groups_statistics<S: Into<GroupId>>(
        &mut self,
        groups: impl IntoIterator<Item = S>,
    ) -> Result<BTreeMap<GroupId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("group")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param(
                "group",
                groups.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<GroupId, StatisticComments>>>()
            .await
    }

    pub async fn get_manga_statistics(
        &mut self,
        id: impl Into<MangaId>,
    ) -> Result<BTreeMap<MangaId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("manga")
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<MangaId, StatisticComments>>>()
            .await
    }

    pub async fn find_manga_statistics<S: Into<MangaId>>(
        &mut self,
        manga: impl IntoIterator<Item = S>,
    ) -> Result<BTreeMap<MangaId, Comments>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Statistics))
            .join("manga")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param(
                "manga",
                manga.into_iter().map(|v| v.into()).collect::<Vec<_>>(),
            )
            .send()
            .await?;

        res.manga_dex_response::<Statistics<BTreeMap<MangaId, StatisticComments>>>()
            .await
    }
}
