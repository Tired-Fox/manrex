use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{cover::*, Data, Paginated},
    uuid::CoverId,
    Client, Error,
};

// ---[ Cover Endpoints ]---
impl Client {
    pub async fn list_covers<M>(
        &mut self,
        filter: impl Optional<CoverArtFilter, M>,
    ) -> Result<Paginated<Vec<Cover>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Cover))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Cover>>>().await
    }

    pub async fn upload_cover(
        &mut self,
        id: impl Into<CoverId>,
        cover: UploadCover,
    ) -> Result<Cover, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        self.rate_limit.request("upload_cover")?;
        let res = Request::post((MangaDex::Api, Endpoint::Cover))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .multipart(cover.into())
            .send()
            .await?;
        self.rate_limit.update("upload_cover", &res)?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn get_cover<M>(
        &mut self,
        id: impl Into<CoverId>,
        includes: impl Optional<Vec<CoverInclude>, M>,
    ) -> Result<Cover, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Cover))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn edit_cover(
        &mut self,
        id: impl Into<CoverId>,
        cover: EditCover,
    ) -> Result<Cover, Error> {
        self.rate_limit.request("edit_cover")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Cover))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&cover)
            .send()
            .await?;
        self.rate_limit.update("edit_cover", &res)?;

        res.manga_dex_response::<Data<Cover>>().await
    }

    pub async fn delete_cover(&mut self, id: impl Into<CoverId>) -> Result<(), Error> {
        self.rate_limit.request("delete_cover")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Cover))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;
        self.rate_limit.update("delete_cover", &res)?;

        res.manga_dex_response::<()>().await
    }
}
