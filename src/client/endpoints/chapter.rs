use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{chapter::*, Data, Paginated},
    uuid::ChapterId,
    Client, Error,
};

// ---[ Chapter Endpoints ]---
impl Client {
    pub async fn list_chapters<M>(
        &mut self,
        filters: impl Optional<ChapterFilter, M>,
    ) -> Result<Paginated<Vec<Chapter>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Chapter))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Chapter>>>().await
    }

    pub async fn get_chapter(&mut self, id: impl Into<ChapterId>) -> Result<Chapter, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Chapter))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<Chapter>>().await
    }

    pub async fn update_chapter(
        &mut self,
        id: impl Into<ChapterId>,
        chapter: UpdateChapter,
    ) -> Result<Chapter, Error> {
        self.rate_limit.request("update_chapter")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Chapter))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&chapter)
            .send()
            .await?;
        self.rate_limit.update("update_chapter", &res)?;

        res.manga_dex_response::<Data<Chapter>>().await
    }

    pub async fn delete_chapter(&mut self, id: impl Into<ChapterId>) -> Result<(), Error> {
        self.rate_limit.request("delete_chapter")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Chapter))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;
        self.rate_limit.update("delete_chapter", &res)?;

        res.manga_dex_response::<()>().await
    }
}
