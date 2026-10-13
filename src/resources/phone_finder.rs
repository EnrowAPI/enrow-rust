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

    /// Launches a single search. Read its result with `get`, or wait for it with `find_and_wait`.
    pub async fn find(&self, params: &PhoneFindParams) -> Result<PhoneFindResponse> {
        self.http.post("/phone/single", params).await
    }

    /// Launches a single search, then polls its result until it is no longer `ongoing`.
    ///
    /// When the timeout elapses first, the last result is returned, still `ongoing`.
    pub async fn find_and_wait(&self, params: &PhoneFindParams, opts: Option<PollOptions>) -> Result<PhoneFindResult> {
        let search = self.find(params).await?;
        let opts = opts.unwrap_or_default();
        crate::resources::poll(
            || self.get(&search.id),
            |r| r.qualification != "ongoing",
            opts,
        ).await
    }

    /// Result of a single search, with `qualification` `ongoing` while it runs.
    pub async fn get(&self, id: &str) -> Result<PhoneFindResult> {
        self.http.get_with_query("/phone/single", &[("id", id)]).await
    }

    /// Launches a bulk search. Read its results with `get_bulk`.
    pub async fn find_bulk(&self, params: &PhoneFindBulkParams) -> Result<PhoneBulkResponse> {
        self.http.post("/phone/bulk", params).await
    }

    /// Results of a bulk search, with `general.status` `ongoing` while it runs.
    pub async fn get_bulk(&self, id: &str) -> Result<PhoneBulkResult> {
        self.http.get_with_query("/phone/bulk", &[("id", id)]).await
    }
}
