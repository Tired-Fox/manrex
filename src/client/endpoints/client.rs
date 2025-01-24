use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{client::*, Data, Paginated},
    Client, Error,
};

// ---[ Client Endpoints ]---
impl Client {
    /// Get a list of clients based on the provided filters
    pub async fn get_clients<M>(
        &mut self,
        filters: impl Optional<ClientFilter, M>,
    ) -> Result<Paginated<Vec<ApiClient>>, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filters.optional())
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<ApiClient>>>().await
    }

    /// Create a new personal client
    pub async fn create_client<M>(&mut self, name: impl std::fmt::Display, description: impl Optional<String, M>) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let mut body = json!({
            "name": name.to_string(),
            "profile": "personal",
            "version": 1
        });

        if let Some(description) = description.optional() {
            body
                .as_object_mut()
                .unwrap()
                .insert("description".into(), serde_json::Value::String(description));
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&body)
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }
    
    /// Delete a client
    pub async fn delete_client(&mut self, id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    /// Edit a client's version and description
    pub async fn edit_client(&mut self, id: impl std::fmt::Display, version: usize, description: impl std::fmt::Display) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "description": description.to_string(),
                "version": version,
            }))
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }

    /// Get a client by it's id
    pub async fn get_client_by_id<M>(&mut self, id: impl std::fmt::Display, includes: impl Optional<Vec<ClientInclude>, M>) -> Result<ApiClient, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .param_opt("includes", includes.optional())
            .send()
            .await?;

        res.manga_dex_response::<Data<ApiClient>>().await
    }

    /// Get a client's secret
    pub async fn get_secret_by_client_id(&mut self, id: impl std::fmt::Display) -> Result<String, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .join("secret")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Data<String>>().await
    }

    /// Regenerate a clients secret
    pub async fn regenerate_client_secret(&mut self, id: impl std::fmt::Display) -> Result<String, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Client))
            .join(id.to_string())
            .join("secret")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({}))
            .send()
            .await?;

        res.manga_dex_response::<Data<String>>().await
    }
}
