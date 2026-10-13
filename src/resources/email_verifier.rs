use crate::client::HttpClient;
use crate::error::Result;
use crate::types::*;

pub struct EmailVerifier {
    http: HttpClient,
}

impl EmailVerifier {
    pub(crate) fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Launches a single verification. Read its result with `get`.
    pub async fn single(&self, params: &VerifySingleParams) -> Result<VerifySingleResponse> {
        self.http.post("/email/verify/single", params).await
    }

    /// Result of a single verification, with `qualification` `ongoing` while it runs.
    pub async fn get(&self, id: &str) -> Result<VerifySingleResult> {
        self.http.get_with_query("/email/verify/single", &[("id", id)]).await
    }

    /// Launches a bulk verification. Read its results with `get_bulk`.
    pub async fn bulk(&self, params: &VerifyBulkParams) -> Result<VerifyBulkResponse> {
        self.http.post("/email/verify/bulk", params).await
    }

    /// Results of a bulk verification, with `general.status` `ongoing` while it runs.
    pub async fn get_bulk(&self, id: &str) -> Result<VerifyBulkResult> {
        self.http.get_with_query("/email/verify/bulk", &[("id", id)]).await
    }
}
