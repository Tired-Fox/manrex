use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{scanlation_group::*, Data, Paginated},
    uuid::GroupId,
    Client, Error,
};

// ---[ Scanlation Group Endpoints ]---
impl Client {
    pub async fn list_scanlation_groups<M>(
        &mut self,
        filter: impl Optional<ScanlationGroupFilter, M>,
    ) -> Result<Paginated<ScanlationGroup>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Group))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<ScanlationGroup>>()
            .await
    }

    pub async fn create_scanlation_group(
        &mut self,
        group: CreateScanlationGroup,
    ) -> Result<ScanlationGroup, Error> {
        self.rate_limit.request("create_scanlation_group")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Group))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&group)
            .send()
            .await?;
        self.rate_limit.update("create_scanlation_group", &res)?;

        res.manga_dex_response::<Data<ScanlationGroup>>().await
    }

    pub async fn get_scanlation_group<M>(
        &mut self,
        id: impl Into<GroupId>,
        includes: impl Optional<Vec<ScanlationGroupInclude>, M>,
    ) -> Result<ScanlationGroup, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Group))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<ScanlationGroup>>().await
    }

    pub async fn update_scanlation_group<M>(
        &mut self,
        id: impl Into<GroupId>,
        group: UpdateScanlationGroup,
    ) -> Result<ScanlationGroup, Error> {
        self.rate_limit.request("update_scanlation_group")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Group))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&group)
            .send()
            .await?;
        self.rate_limit.update("update_scanlation_group", &res)?;

        res.manga_dex_response::<Data<ScanlationGroup>>().await
    }

    pub async fn delete_scanlation_group(&mut self, id: impl Into<GroupId>) -> Result<(), Error> {
        self.rate_limit.request("delete_scanlation_group")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Group))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;
        self.rate_limit.update("delete_scanlation_group", &res)?;

        res.manga_dex_response::<()>().await
    }

    pub async fn follow_scanlation_group(&mut self, id: impl Into<GroupId>) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Group))
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

    pub async fn unfollow_scanlation_group(&mut self, id: impl Into<GroupId>) -> Result<(), Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Group))
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
}
