use crate::client::HttpClient;
use crate::error::Result;
use crate::types::AccountInfo;

pub struct AccountResource {
    http: HttpClient,
}

impl AccountResource {
    pub(crate) fn new(http: HttpClient) -> Self {
        Self { http }
    }

    pub async fn info(&self) -> Result<AccountInfo> {
        self.http.get("/account/info").await
    }
}
