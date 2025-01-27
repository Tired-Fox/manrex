use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{author::*, Data, Paginated},
    uuid::AuthorId,
    Client, Error,
};

// ---[ Author Endpoints ]---
impl Client {
    pub async fn list_authors<M>(
        &mut self,
        filters: impl Optional<AuthorFilter, M>,
    ) -> Result<Paginated<Vec<Author>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Author))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<Author>>>().await
    }

    pub async fn create_author(&mut self, author: CreateAuthor) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Author))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&author)
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn get_author<M>(
        &mut self,
        id: impl Into<AuthorId>,
        includes: impl Optional<Vec<AuthorInclude>, M>,
    ) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Author))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn update_author(
        &mut self,
        id: impl Into<AuthorId>,
        author: UpdateAuthor,
    ) -> Result<Author, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::put((MangaDex::Api, Endpoint::Author))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .json(&author)
            .send()
            .await?;

        res.manga_dex_response::<Data<Author>>().await
    }

    pub async fn delete_author(&mut self, id: impl Into<AuthorId>) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Author))
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
