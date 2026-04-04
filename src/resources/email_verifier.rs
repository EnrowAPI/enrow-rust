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

    pub async fn single(&self, params: &VerifySingleParams) -> Result<VerifySingleResult> {
        self.http.post("/email/verify/single", params).await
    }

    pub async fn get(&self, id: &str) -> Result<VerifySingleResult> {
        self.http.get_with_query("/email/verify/single", &[("id", id)]).await
    }

    pub async fn bulk(&self, params: &VerifyBulkParams) -> Result<VerifyBulkResponse> {
        self.http.post("/email/verify/bulk", params).await
    }

    pub async fn get_bulk(&self, id: &str) -> Result<VerifyBulkResult> {
        self.http.get_with_query("/email/verify/bulk", &[("id", id)]).await
    }
}
