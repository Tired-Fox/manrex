use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{custom_list::*, Data, Paginated},
    Client, Error, ListId, MangaId, UserId,
};

// ---[ Custom List Endpoints ]---
impl Client {
    pub async fn create_list(
        &mut self,
        custom_list: CreateCustomList,
    ) -> Result<CustomList, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::List))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&custom_list)
            .send()
            .await?;

        res.manga_dex_response::<Data<CustomList>>().await
    }

    pub async fn get_list<M1>(
        &mut self,
        id: impl Into<ListId>,
        includes: impl Optional<Vec<ListInclude>, M1>
    ) -> Result<CustomList, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::List))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<CustomList>>().await
    }

    pub async fn update_list(
        &mut self,
        id: impl Into<ListId>,
        custom_list: UpdateCustomList,
    ) -> Result<CustomList, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::List))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&custom_list)
            .send()
            .await?;

        res.manga_dex_response::<Data<CustomList>>().await
    }

    pub async fn delete_list(
        &mut self,
        id: impl Into<ListId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::List))
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

    pub async fn follow_list(
        &mut self,
        id: impl Into<ListId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::List))
            .join(id.into().as_ref())
            .join("follow")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({}))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn unfollow_list(
        &mut self,
        id: impl Into<ListId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::List))
            .join(id.into().as_ref())
            .join("follow")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&json!({}))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn add_manga_to_list(
        &mut self,
        manga: impl Into<MangaId>,
        list: impl Into<ListId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Manga))
            .join(manga.into().as_ref())
            .join("list")
            .join(list.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn remove_manga_from_list(
        &mut self,
        manga: impl Into<MangaId>,
        list: impl Into<ListId>,
    ) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Manga))
            .join(manga.into().as_ref())
            .join("list")
            .join(list.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn get_lists<M1, M2>(
        &mut self,
        limit: impl Optional<usize, M1>,
        offset: impl Optional<usize, M2>,
    ) -> Result<Paginated<CustomList>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .join("list")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("limit", limit.optional())
            .param_opt("offset", offset.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<CustomList>>().await
    }

    pub async fn get_users_lists<M1, M2>(
        &mut self,
        id: impl Into<UserId>,
        limit: impl Optional<usize, M1>,
        offset: impl Optional<usize, M2>,
    ) -> Result<Paginated<CustomList>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .join(id.into().as_ref())
            .join("list")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("limit", limit.optional())
            .param_opt("offset", offset.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<CustomList>>().await
    }
}
