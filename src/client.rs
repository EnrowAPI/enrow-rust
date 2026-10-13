use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{EnrowError, Result};
use crate::resources::{AccountResource, EmailFinder, EmailVerifier, PhoneFinder};

const BASE_URL: &str = "https://api.enrow.io";

pub struct Enrow {
    pub email: EmailFinder,
    pub verify: EmailVerifier,
    pub phone: PhoneFinder,
    pub account: AccountResource,
}

impl Enrow {
    pub fn new(api_key: &str) -> Self {
        Self::with_base_url(api_key, BASE_URL)
    }

    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let http = HttpClient::new(api_key, base_url);
        Self {
            email: EmailFinder::new(http.clone()),
            verify: EmailVerifier::new(http.clone()),
            phone: PhoneFinder::new(http.clone()),
            account: AccountResource::new(http),
        }
    }
}

#[derive(Clone)]
pub(crate) struct HttpClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl HttpClient {
    fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.client
            .get(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn get_with_query<T: DeserializeOwned>(&self, path: &str, query: &[(&str, &str)]) -> Result<T> {
        let resp = self.client
            .get(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .query(query)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    pub(crate) async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let resp = self.client
            .post(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let status = resp.status().as_u16();

        if status >= 400 {
            let body: serde_json::Value = resp.json().await.unwrap_or_default();
            return Err(EnrowError::from_response(status, &body));
        }

        Ok(resp.json().await?)
    }
}
