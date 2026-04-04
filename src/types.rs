use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub name: String,
    pub domain: String,
}

// ── Email Finder ──

#[derive(Debug, Serialize, Default)]
pub struct EmailFindParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SearchSettings>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SearchSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailFindResult {
    pub id: String,
    pub status: String,
    pub email: Option<String>,
    pub qualification: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<Company>,
    pub verified: Option<bool>,
    pub credits_used: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct EmailFindBulkParams {
    pub searches: Vec<EmailFindParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SearchSettings>,
}

#[derive(Debug, Deserialize)]
pub struct EmailFindBulkResponse {
    pub batch_id: String,
    pub total: u32,
    pub status: String,
    pub credits_used: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct EmailFindBulkResult {
    pub batch_id: String,
    pub status: String,
    pub total: u32,
    pub completed: Option<u32>,
    pub credits_used: Option<f64>,
    pub results: Option<Vec<EmailFindResult>>,
}

// ── Email Verifier ──

#[derive(Debug, Serialize)]
pub struct VerifySingleParams {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WebhookSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VerifyChecks {
    pub syntax_valid: bool,
    pub domain_valid: bool,
    pub mx_found: bool,
    pub smtp_valid: bool,
    pub mailbox_exists: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VerifyMetadata {
    pub is_disposable: bool,
    pub is_role: bool,
    pub is_free: bool,
    pub is_catch_all: bool,
    pub provider: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VerifySingleResult {
    pub id: String,
    pub email: String,
    pub qualification: String,
    pub is_deliverable: bool,
    pub checks: VerifyChecks,
    pub metadata: VerifyMetadata,
    pub credits_used: f64,
}

#[derive(Debug, Serialize)]
pub struct VerifyBulkParams {
    pub emails: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyBulkResponse {
    pub batch_id: String,
    pub total: u32,
    pub status: String,
    pub credits_used: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyBulkResult {
    pub batch_id: String,
    pub status: String,
    pub total: u32,
    pub completed: Option<u32>,
    pub credits_used: Option<f64>,
    pub results: Option<Vec<VerifySingleResult>>,
}

// ── Phone Finder ──

#[derive(Debug, Serialize, Default)]
pub struct PhoneFindParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Deserialize)]
pub struct PhoneFindResponse {
    pub id: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PhoneFindResult {
    pub id: String,
    pub qualification: String,
    pub phone: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PhoneFindBulkParams {
    pub searches: Vec<PhoneFindParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Deserialize)]
pub struct PhoneBulkResponse {
    pub batch_id: String,
    pub total: u32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct PhoneBulkResult {
    pub batch_id: String,
    pub status: String,
    pub total: u32,
    pub results: Option<Vec<PhoneBulkItem>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PhoneBulkItem {
    pub index: u32,
    pub qualification: String,
    pub phone: Option<String>,
    pub country_code: Option<String>,
}

// ── Reverse Email ──

#[derive(Debug, Serialize)]
pub struct ReverseEmailParams {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReverseEmailResult {
    pub id: String,
    pub status: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<Company>,
    pub linkedin_url: Option<String>,
    pub credits_used: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ReverseEmailBulkEmail {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ReverseEmailBulkParams {
    pub emails: Vec<ReverseEmailBulkEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

#[derive(Debug, Deserialize)]
pub struct ReverseEmailBulkResponse {
    pub id: String,
    pub status: String,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct ReverseEmailBulkResult {
    pub id: String,
    pub status: String,
    pub total: u32,
    pub completed: Option<u32>,
    pub credits_used: Option<f64>,
    pub results: Option<Vec<ReverseEmailBulkItem>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReverseEmailBulkItem {
    pub email: String,
    pub status: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<Company>,
    pub linkedin_url: Option<String>,
    pub index: u32,
}

// ── Account ──

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    pub credits: f64,
    pub webhooks: Vec<String>,
}

// ── Polling ──

pub struct PollOptions {
    pub poll_interval: std::time::Duration,
    pub timeout: std::time::Duration,
}

impl Default for PollOptions {
    fn default() -> Self {
        Self {
            poll_interval: std::time::Duration::from_secs(2),
            timeout: std::time::Duration::from_secs(30),
        }
    }
}
