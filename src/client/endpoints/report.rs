use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::{
    client::{Endpoint, MangaDex, Optional, Request, CLIENT_NAME, CLIENT_VERSION}, error::ResponseToError, model::{report::*, Category, Paginated}, Client, Error
};

// ---[ Report Endpoints ]---
impl Client {
    pub async fn list_report_reasons(&mut self, category: Category) -> Result<Paginated<Vec<ReportReason>>, Error> {
        self.rate_limit.request("")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Report))
            .join("reason")
            .join(category.to_string())
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .send()
            .await?;

        res.manga_dex_response::<Paginated<Vec<ReportReason>>>().await
    }

    pub async fn list_user_reports<M>(&mut self, filter: impl Optional<ReportFilter, M>) -> Result<Paginated<Vec<Report>>, Error> {
        self.rate_limit.request("list_user_reports")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::get((MangaDex::Api, Endpoint::Report))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .params_opt(filter.optional())
            .send()
            .await?;

        self.rate_limit.update("list_user_reports", &res)?;

        res.manga_dex_response::<Paginated<Vec<Report>>>().await
    }

    pub async fn create_report(&mut self, report: CreateReport) -> Result<(), Error> {
        self.rate_limit.request("create_report")?;
        if self.oauth().expired()? {
            self.oauth.refresh().await?;
        }

        let res = Request::post((MangaDex::Api, Endpoint::Report))
            .header(USER_AGENT, format!("{CLIENT_NAME}/{CLIENT_VERSION}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.oauth().access_token()))
            .json(&report)
            .send()
            .await?;

        self.rate_limit.update("create_report", &res)?;

        res.manga_dex_response::<()>().await
    }
}
