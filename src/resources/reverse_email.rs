use crate::client::HttpClient;
use crate::error::Result;
use crate::types::*;

pub struct ReverseEmailResource {
    http: HttpClient,
}

impl ReverseEmailResource {
    pub(crate) fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn find(&self, params: &ReverseEmailParams) -> Result<ReverseEmailResult> {
        self.http.post("/reverse-email/single", params).await
    }

    pub async fn find_and_wait(&self, params: &ReverseEmailParams, opts: Option<PollOptions>) -> Result<ReverseEmailResult> {
        let result: ReverseEmailResult = self.http.post("/reverse-email/single", params).await?;
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

    pub async fn get(&self, id: &str) -> Result<ReverseEmailResult> {
        self.http.get(&format!("/reverse-email/single/{}", id)).await
    }

    pub async fn find_bulk(&self, params: &ReverseEmailBulkParams) -> Result<ReverseEmailBulkResponse> {
        self.http.post("/reverse-email/bulk", params).await
    }

    pub async fn get_bulk(&self, id: &str) -> Result<ReverseEmailBulkResult> {
        self.http.get(&format!("/reverse-email/bulk/{}", id)).await
    }
}
