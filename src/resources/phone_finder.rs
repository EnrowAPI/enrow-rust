use crate::client::HttpClient;
use crate::error::Result;
use crate::types::*;

pub struct PhoneFinder {
    http: HttpClient,
}

impl PhoneFinder {
    pub(crate) fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn find(&self, params: &PhoneFindParams) -> Result<PhoneFindResponse> {
        self.http.post("/phone/single", params).await
    }

    pub async fn find_and_wait(&self, params: &PhoneFindParams, opts: Option<PollOptions>) -> Result<PhoneFindResult> {
        let resp: PhoneFindResponse = self.http.post("/phone/single", params).await?;
        let opts = opts.unwrap_or_default();
        crate::resources::poll(
            || self.get(&resp.id),
            |r| r.qualification != "ongoing",
            opts,
        ).await
    }

    pub async fn get(&self, id: &str) -> Result<PhoneFindResult> {
        self.http.get_with_query("/phone/single", &[("id", id)]).await
    }

    pub async fn find_bulk(&self, params: &PhoneFindBulkParams) -> Result<PhoneBulkResponse> {
        self.http.post("/phone/bulk", params).await
    }

    pub async fn get_bulk(&self, id: &str) -> Result<PhoneBulkResult> {
        self.http.get_with_query("/phone/bulk", &[("id", id)]).await
    }
}
