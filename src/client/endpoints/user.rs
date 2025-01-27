use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{user::*, Data, Paginated},
    uuid::UserId,
    Client, Error,
};

// ---[ User Endpoints ]---
impl Client {
    pub async fn list_users<M>(
        &mut self,
        filter: impl Optional<UserFilter, M>,
    ) -> Result<Paginated<Vec<User>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .params_opt(filter.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<User>>>().await
    }

    pub async fn get_user(&mut self, id: impl Into<UserId>) -> Result<User, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .join(id.into().as_ref())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<User>>().await
    }

    pub async fn get_user_details(&mut self) -> Result<User, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::User))
            .join("me")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.oauth().access_token()),
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<User>>().await
    }
}
