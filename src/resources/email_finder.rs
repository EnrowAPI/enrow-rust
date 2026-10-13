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

    /// Launches a single search. Read its result with `get`, or wait for it with `find_and_wait`.
    pub async fn find(&self, params: &EmailFindParams) -> Result<EmailFindResponse> {
        self.http.post("/email/find/single", params).await
    }

    /// Launches a single search, then polls its result until it is no longer `ongoing`.
    ///
    /// When the timeout elapses first, the last result is returned, still `ongoing`.
    pub async fn find_and_wait(&self, params: &EmailFindParams, opts: Option<PollOptions>) -> Result<EmailFindResult> {
        let search = self.find(params).await?;
        let opts = opts.unwrap_or_default();
        crate::resources::poll(
            || self.get(&search.id),
            |r| r.qualification != "ongoing",
            opts,
        ).await
    }

    /// Result of a single search, with `qualification` `ongoing` while it runs.
    pub async fn get(&self, id: &str) -> Result<EmailFindResult> {
        self.http.get_with_query("/email/find/single", &[("id", id)]).await
    }

    /// Launches a bulk search. Read its results with `get_bulk`.
    pub async fn find_bulk(&self, params: &EmailFindBulkParams) -> Result<EmailFindBulkResponse> {
        self.http.post("/email/find/bulk", params).await
    }

    /// Results of a bulk search, with `general.status` `ongoing` while it runs.
    pub async fn get_bulk(&self, id: &str) -> Result<EmailFindBulkResult> {
        self.http.get_with_query("/email/find/bulk", &[("id", id)]).await
    }
}
