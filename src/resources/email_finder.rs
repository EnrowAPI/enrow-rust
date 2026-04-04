use crate::client::HttpClient;
use crate::error::Result;
use crate::types::*;

pub struct EmailFinder {
    http: HttpClient,
}

impl EmailFinder {
    pub(crate) fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn find(&self, params: &EmailFindParams) -> Result<EmailFindResult> {
        self.http.post("/email/find/single", params).await
    }

    pub async fn find_and_wait(&self, params: &EmailFindParams, opts: Option<PollOptions>) -> Result<EmailFindResult> {
        let result: EmailFindResult = self.http.post("/email/find/single", params).await?;
        if result.status == "pending" || result.status == "processing" {
            let opts = opts.unwrap_or_default();
            return crate::resources::poll(
                || self.get(&result.id),
                |r| r.status != "pending" && r.status != "processing",
                opts,
            ).await;
        }
        Ok(result)
    }

    pub async fn get(&self, id: &str) -> Result<EmailFindResult> {
        self.http.get_with_query("/email/find/single", &[("id", id)]).await
    }

    pub async fn find_bulk(&self, params: &EmailFindBulkParams) -> Result<EmailFindBulkResponse> {
        self.http.post("/email/find/bulk", params).await
    }

    pub async fn get_bulk(&self, id: &str) -> Result<EmailFindBulkResult> {
        self.http.get_with_query("/email/find/bulk", &[("id", id)]).await
    }
}
