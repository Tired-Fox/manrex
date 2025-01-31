use std::collections::BTreeMap;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{chapter::Chapter, manga::*, Data, Paginated, Relation, Status},
    uuid::{GroupId, MangaId},
    Client, Error,
};

// ---[ Manga Endpoints ]---
impl Client {
    pub async fn list_manga<M>(
        &mut self,
        filter: impl Optional<MangaFilter, M>,
    ) -> Result<Paginated<Vec<Manga>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Manga>>>().await
    }

    pub async fn get_manga_volumes_and_chapters<M1, M2>(
        &mut self,
        id: impl Into<MangaId>,
        translated_languages: impl Optional<Vec<String>, M1>,
        groups: impl Optional<Vec<GroupId>, M2>,
    ) -> Result<BTreeMap<String, Volume>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("aggregate")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("translatedLanguage", translated_languages.optional())
            .param_opt("groups", groups.optional())
            .send()
            .await?;

        res.manga_dex_response::<Volumes<BTreeMap<String, Volume>>>()
            .await
    }

    pub async fn get_manga<M>(
        &mut self,
        id: impl Into<MangaId>,
        includes: impl Optional<Vec<MangaInclude>, M>,
    ) -> Result<Manga, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn create_manga(&mut self, manga: CreateManga) -> Result<Manga, Error> {
        self.rate_limit.request("create_manga")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&manga)
            .send()
            .await?;
        self.rate_limit.update("create_manga", &res)?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn follow_manga(&mut self, id: impl Into<MangaId>) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("follow")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn update_manga(
        &mut self,
        id: impl Into<MangaId>,
        manga: UpdateManga,
    ) -> Result<Manga, Error> {
        self.rate_limit.request("update_manga")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&manga)
            .send()
            .await?;
        self.rate_limit.update("update_manga", &res)?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn delete_manga(&mut self, id: impl Into<MangaId>) -> Result<(), Error> {
        self.rate_limit.request("delete_manga")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;
        self.rate_limit.update("delete_manga", &res)?;

        res.manga_dex_response::<()>().await
    }

    pub async fn unfollow_manga(&mut self, id: impl Into<MangaId>) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("follow")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn get_manga_feed<M>(
        &mut self,
        id: impl Into<MangaId>,
        filter: impl Optional<FeedFilter, M>,
    ) -> Result<Paginated<Vec<Chapter>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("feed")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Chapter>>>().await
    }

    pub async fn get_random_manga<M>(
        &mut self,
        filter: impl Optional<RandomMangaFilter, M>,
    ) -> Result<Manga, Error> {
        self.rate_limit.request("get_random_manga")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("random")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;
        self.rate_limit.update("get_random_manga", &res)?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn get_manga_tag_list(&mut self) -> Result<Paginated<Vec<Tag>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("tag")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Tag>>>().await
    }

    pub async fn get_manga_reading_statuses<M>(
        &mut self,
        status: impl Optional<Status, M>,
    ) -> Result<BTreeMap<String, Status>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("status")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("status", status.optional().map(|v| v.to_string()))
            .send()
            .await?;

        res.manga_dex_response::<Statuses<BTreeMap<String, Status>>>()
            .await
    }

    pub async fn get_manga_reading_status(
        &mut self,
        id: impl Into<MangaId>,
    ) -> Result<Status, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("status")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<DataStatus>().await
    }

    pub async fn update_manga_reading_status<M>(
        &mut self,
        id: impl Into<MangaId>,
        status: impl Optional<Status, M>,
    ) -> Result<Status, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("status")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({
                "status": status.optional()
            }))
            .send()
            .await?;

        res.manga_dex_response::<DataStatus>().await
    }

    pub async fn get_specific_manga_draft<M>(
        &mut self,
        id: impl Into<MangaId>,
        includes: impl Optional<Vec<MangaInclude>, M>,
    ) -> Result<Status, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("draft")
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<DataStatus>().await
    }

    pub async fn submit_manga_draft<M>(
        &mut self,
        id: impl Into<MangaId>,
        version: usize,
    ) -> Result<Manga, Error> {
        self.rate_limit.request("submit_manga_draft")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join("draft")
            .join(id.into().as_ref())
            .join("commit")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({
                "version": version
            }))
            .send()
            .await?;
        self.rate_limit.update("submit_manga_draft", &res)?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn list_manga_drafts<M>(
        &mut self,
        filter: impl Optional<DraftFilter, M>,
    ) -> Result<Manga, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join("draft")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Manga>>().await
    }

    pub async fn get_manga_relation_list<M>(
        &mut self,
        id: impl Into<MangaId>,
        includes: impl Optional<Vec<MangaInclude>, M>,
    ) -> Result<Paginated<Vec<MangaRelation>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("relation")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<MangaRelation>>>()
            .await
    }

    pub async fn create_manga_relation(
        &mut self,
        id: impl Into<MangaId>,
        target: impl Into<MangaId>,
        relation: Relation,
    ) -> Result<MangaRelation, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("relation")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({
                "targetManga": target.into().as_ref(),
                "relation": relation,
            }))
            .send()
            .await?;

        res.manga_dex_response::<Data<MangaRelation>>().await
    }

    pub async fn delete_manga_relation(
        &mut self,
        id: impl Into<MangaId>,
        target: impl Into<MangaId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(id.into().as_ref())
            .join("relation")
            .join(target.into().as_ref())
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
