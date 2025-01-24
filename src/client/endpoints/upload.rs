use std::path::Path;

use reqwest::{header::{AUTHORIZATION, USER_AGENT}, multipart};
use serde_json::json;

use crate::{
    client::{Endpoint, MangaDex, Request, CLIENT_NAME, CLIENT_VERSION},
    error::ResponseToError,
    model::{upload::*, Data},
    Client, Error,
};

// ---[ Upload Endpoints ]---
impl Client {
    pub async fn get_upload_session(&mut self) -> Result<UploadSession, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Upload))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_template::<UploadSession>().await
    }

    pub async fn start_upload_session<S: std::fmt::Display>(&mut self, groups: impl IntoIterator<Item=S>, manga: impl std::fmt::Display) -> Result<UploadSession, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Upload))
            .join("begin")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "groups": groups.into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                "manga": manga.to_string()
            }))
            .send()
            .await?;

        res.manga_dex_template::<UploadSession>().await
    }

    pub async fn start_edit_chapter(&mut self, id: impl std::fmt::Display, version: usize) -> Result<UploadSession, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Upload))
            .join("begin")
            .join(id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "version": version
            }))
            .send()
            .await?;

        res.manga_dex_template::<UploadSession>().await
    }

    pub async fn upload_image(&mut self, session_id: impl std::fmt::Display, file: impl AsRef<Path>) -> Result<FileUploadSession, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Upload))
            .join(session_id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .multipart(
                multipart::Form::new()
                    .file("file", file)
                    .await?
            )
            .send()
            .await?;

        res.manga_dex_response::<Data<FileUploadSession>>().await
    }

    pub async fn commit_upload_session<S: std::fmt::Display>(
        &mut self,
        session_id: impl std::fmt::Display,
        chapter_draft: ChapterDraft,
        page_order: impl IntoIterator<Item=S>
    ) -> Result<FileUploadSession, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Upload))
            .join(session_id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "chapterDraft": chapter_draft,
                "pageOrder": page_order.into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
            }))
            .send()
            .await?;

        res.manga_dex_response::<Data<FileUploadSession>>().await
    }

    pub async fn abandon_upload_session(&mut self, session_id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Upload))
            .join(session_id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn delete_uploaded_image(&mut self, session_id: impl std::fmt::Display, file_session_id: impl std::fmt::Display) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Upload))
            .join(session_id.to_string())
            .join(file_session_id.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn delete_uploaded_images<S: std::fmt::Display>(&mut self, session_id: impl std::fmt::Display, file_session_ids: impl IntoIterator<Item=S>) -> Result<(), Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::delete((MangaDex::Api, Endpoint::Upload))
            .join(session_id.to_string())
            .join("batch")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&file_session_ids.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .send()
            .await?;

        res.manga_dex_response::<()>().await
    }

    pub async fn check_manga_needs_approval(&mut self, manga: impl std::fmt::Display, locale: impl std::fmt::Display) -> Result<bool, Error> {
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Upload))
            .join("check-approval-required")
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&json!({
                "manga": manga.to_string(),
                "locale": locale.to_string(),
            }))
            .send()
            .await?;

        res.manga_dex_response::<RequiresApproval>().await
    }
}
