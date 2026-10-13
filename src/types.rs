use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Settings ──

/// Settings of an Email Finder search, single or bulk.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SearchSettings {
    /// ISO 3166 Alpha-2 code of the country of the search (default `US`), used with `company_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Also return the gender (`male` or `female`) of the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_gender: Option<bool>,
    /// Also return company information (only effective when `country_code` is `FR`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_company_info: Option<bool>,
    /// URL notified with an HTTP POST once the search is finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

/// Settings of an Email Verifier or Phone Finder request.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WebhookSettings {
    /// URL notified with an HTTP POST once the request is finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
}

// ── Shared response parts ──

/// Credits debited when an Email Verifier or Phone Finder request is launched.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CreditsInfo {
    /// Credits debited.
    pub amount: Option<f64>,
    /// Where the credits came from: `sub`, `paygo` or `mixed`.
    pub source: Option<String>,
    /// Split between subscription and pay-as-you-go credits.
    pub split: Option<CreditsSplit>,
}

/// Split of the debited credits between subscription and pay-as-you-go credits.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CreditsSplit {
    #[serde(rename = "fromSub")]
    pub from_sub: Option<f64>,
    #[serde(rename = "fromPaygo")]
    pub from_paygo: Option<f64>,
}

/// Batch metadata (`general`) of a bulk result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct BulkGeneral {
    /// Batch id.
    pub id: String,
    /// `ongoing`, `completed` or `failed`.
    pub status: String,
    /// The batch-level `custom` sent with the request, returned as-is.
    pub custom: Option<Value>,
}

// ── Email Finder ──

/// Body of `POST /email/find/single`, also used for each search of a bulk.
///
/// Give `fullname` with `company_domain` or `company_name`, or a `linkedin_url`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct EmailFindParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullname: Option<String>,
    /// LinkedIn profile URL, to search from the profile instead of a name and a company.
    /// Single searches only, and ignored when `fullname` and a company are given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    /// Your own data (an object, or a string in a bulk), returned as-is with the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    /// Single searches only: a bulk takes its settings in `EmailFindBulkParams`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SearchSettings>,
}

/// Response of `POST /email/find/single`: the search is launched, its result comes later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindResponse {
    /// Search id, to read the result with `EmailFinder::get`.
    pub id: String,
    pub message: Option<String>,
    /// Credits debited at launch. The final cost is in the result.
    pub credits_used: Option<f64>,
}

/// Result of a single email search (`GET /email/find/single`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindResult {
    /// `valid` (email found), `invalid` (not found) or `ongoing` (still running).
    pub qualification: String,
    pub email: Option<String>,
    /// Contact and company information.
    pub info: Option<EmailFindInfo>,
    /// The `custom` sent with the search, returned as-is.
    pub custom: Option<Value>,
    /// Final cost of the search, once refunds are applied.
    pub credits_used: Option<f64>,
    /// Search from a LinkedIn profile URL only: `found`, `profile_not_found`,
    /// `no_current_company`, `email_not_found` or `lookup_failed`.
    pub linkedin_status: Option<String>,
    /// Search from a LinkedIn profile URL only: the URL sent.
    pub linkedin_url: Option<String>,
}

/// Contact and company information of a single email search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindInfo {
    pub fullname: Option<String>,
    #[serde(rename = "firstname")]
    pub first_name: Option<String>,
    #[serde(rename = "lastname")]
    pub last_name: Option<String>,
    /// `male` or `female`, when `retrieve_gender` was set.
    pub gender: Option<String>,
    pub company_domain: Option<String>,
    pub company_name: Option<String>,
    /// Additional company data, when available.
    #[serde(rename = "companyInfo")]
    pub company_info: Option<Value>,
}

/// Body of `POST /email/find/bulk`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct EmailFindBulkParams {
    /// Up to 5,000 searches, each with `fullname` and `company_domain` or `company_name`.
    pub searches: Vec<EmailFindParams>,
    /// Your own data for the whole batch, returned as-is in `general.custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SearchSettings>,
}

/// Response of `POST /email/find/bulk`: the batch is launched, its results come later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkResponse {
    /// Batch id, to read the results with `EmailFinder::get_bulk`.
    pub id: String,
    pub message: Option<String>,
    pub credits_used: Option<f64>,
    /// Estimated processing time in minutes (not guaranteed).
    pub estimated_duration: Option<f64>,
}

/// Results of a bulk email search (`GET /email/find/bulk`).
///
/// While the batch runs, `general.status` is `ongoing` and `results` is absent.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkResult {
    pub general: BulkGeneral,
    pub stats: EmailFindBulkStats,
    pub results: Option<Vec<EmailFindBulkItem>>,
}

/// Statistics of a bulk email search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkStats {
    pub finished: Option<u32>,
    pub requested: Option<u32>,
    /// Number of emails found.
    pub valid: Option<u32>,
    pub credits_cost: Option<EmailFindBulkCredits>,
}

/// Credits of a bulk email search: charged at launch, refunded, final cost.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkCredits {
    pub initial: Option<f64>,
    pub refunded: Option<f64>,
    #[serde(rename = "final")]
    pub final_cost: Option<f64>,
}

/// One search of a bulk email search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkItem {
    /// Position of the search in the request (`"0"`, `"1"`...).
    pub index: Option<String>,
    /// `valid` (email found) or `invalid` (not found).
    pub qualification: String,
    pub email: Option<String>,
    /// Only for an email found.
    pub info: Option<EmailFindBulkInfo>,
    /// The parameters of the search, as sent.
    pub raw_params: Option<Value>,
    /// The `custom` of the search, returned as-is.
    pub custom: Option<Value>,
}

/// Contact information of a bulk email search result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EmailFindBulkInfo {
    pub domain: Option<String>,
    #[serde(rename = "firstname")]
    pub first_name: Option<String>,
    #[serde(rename = "lastname")]
    pub last_name: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "companyInfo")]
    pub company_info: Option<Value>,
}

// ── Email Verifier ──

/// Body of `POST /email/verify/single`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct VerifySingleParams {
    pub email: String,
    /// Your own data (usually an object), returned as-is with the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

/// Response of `POST /email/verify/single`: the verification is launched, its result comes later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifySingleResponse {
    /// Verification id, to read the result with `EmailVerifier::get`.
    pub id: String,
    pub message: Option<String>,
    pub credits_used: Option<f64>,
    pub credits: Option<CreditsInfo>,
}

/// Result of a single verification (`GET /email/verify/single`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifySingleResult {
    /// `valid` (deliverable), `invalid` (not deliverable) or `ongoing` (still running).
    pub qualification: String,
    pub email: Option<String>,
    /// The `custom` sent with the verification, returned as-is.
    pub custom: Option<Value>,
}

/// Body of `POST /email/verify/bulk`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct VerifyBulkParams {
    /// Up to 5,000 email addresses, sent as `verifications`.
    #[serde(rename = "verifications")]
    pub emails: Vec<String>,
    /// Your own data for the whole batch, returned as-is in `general.custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

/// Response of `POST /email/verify/bulk`: the batch is launched, its results come later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifyBulkResponse {
    /// Batch id, to read the results with `EmailVerifier::get_bulk`.
    pub id: String,
    pub message: Option<String>,
    pub credits_used: Option<f64>,
    /// Estimated processing time in minutes (not guaranteed).
    pub estimated_duration: Option<f64>,
    pub credits: Option<CreditsInfo>,
}

/// Results of a bulk verification (`GET /email/verify/bulk`).
///
/// While the batch runs, `general.status` is `ongoing` and `results` is absent.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifyBulkResult {
    pub general: BulkGeneral,
    pub stats: VerifyBulkStats,
    pub results: Option<Vec<VerifyBulkItem>>,
}

/// Statistics of a bulk verification.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifyBulkStats {
    /// Only while the batch runs.
    pub finished: Option<u32>,
    pub requested: Option<u32>,
    /// Number of valid emails.
    pub valid: Option<u32>,
    /// Total credits consumed.
    pub credits_cost: Option<f64>,
}

/// One verification of a bulk verification.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VerifyBulkItem {
    pub email: Option<String>,
    /// Identifier of the verification in the batch.
    pub index: Option<String>,
    /// `valid` (deliverable) or `invalid` (not deliverable).
    pub qualification: String,
}

// ── Phone Finder ──

/// Body of `POST /phone/single`, also used for each search of a bulk.
///
/// Give a `linkedin_url` (recommended, it prevails), or `first_name`, `last_name`
/// and `company_domain` or `company_name`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct PhoneFindParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    /// Sent as `firstname`.
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Sent as `lastname`.
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Your own data (usually a string), returned as-is with the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    /// Single searches only: a bulk takes its settings in `PhoneFindBulkParams`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

/// Response of `POST /phone/single`: the search is launched, its result comes later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneFindResponse {
    /// Search id, to read the result with `PhoneFinder::get`.
    pub id: String,
    pub message: Option<String>,
    /// Credits debited at launch, refunded when no number is found.
    pub credits_used: Option<f64>,
    pub credits: Option<CreditsInfo>,
}

/// Result of a single phone search (`GET /phone/single`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneFindResult {
    /// `found`, `not_found` or `ongoing` (still running).
    pub qualification: String,
    /// The phone number, `+`-prefixed, when found.
    pub number: Option<String>,
    /// ISO 3166-1 Alpha-2 country code of the number, when found.
    pub country: Option<String>,
    /// The parameters of the search, as sent.
    pub params: Option<Value>,
    /// The `custom` sent with the search, returned as-is.
    pub custom: Option<Value>,
}

/// Body of `POST /phone/bulk`.
#[derive(Debug, Serialize, Default, Clone)]
pub struct PhoneFindBulkParams {
    /// Up to 3,000 searches.
    pub searches: Vec<PhoneFindParams>,
    /// Your own data for the whole batch, returned as-is in `general.custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<WebhookSettings>,
}

/// Response of `POST /phone/bulk`: the batch is launched, its results come later.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneBulkResponse {
    /// Batch id, to read the results with `PhoneFinder::get_bulk`.
    pub id: String,
    pub message: Option<String>,
    pub credits_used: Option<f64>,
    /// Estimated processing time in minutes (not guaranteed).
    pub estimated_duration: Option<f64>,
    pub credits: Option<CreditsInfo>,
}

/// Results of a bulk phone search (`GET /phone/bulk`).
///
/// While the batch runs, `general.status` is `ongoing` and `results` is absent.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneBulkResult {
    pub general: BulkGeneral,
    pub stats: PhoneBulkStats,
    pub results: Option<Vec<PhoneBulkItem>>,
}

/// Statistics of a bulk phone search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneBulkStats {
    /// Only while the batch runs.
    pub finished: Option<u32>,
    pub requested: Option<u32>,
    /// Number of phone numbers found.
    pub found: Option<u32>,
    /// Total credits consumed.
    pub credits_cost: Option<f64>,
    /// Credits refunded for the searches without a number.
    pub credits_to_refund: Option<f64>,
}

/// One search of a bulk phone search.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PhoneBulkItem {
    /// Position of the search in the request.
    pub index: Option<u32>,
    /// `found` or `not_found`.
    pub qualification: String,
    pub number: Option<String>,
    pub country: Option<String>,
    /// The parameters of the search, as sent.
    pub params: Option<Value>,
}

// ── Account ──

/// Account information (`GET /account/info`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AccountInfo {
    /// Current credit balance.
    pub credits: f64,
    /// Registered webhook URLs, empty when there are none.
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

// Sample bodies copied from the API reference (docs.enrow.io/api-reference).
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::{json, Value};

    fn decode<T: serde::de::DeserializeOwned>(body: &str) -> T {
        serde_json::from_str(body).expect("the documented body should decode")
    }

    fn encode<T: Serialize>(params: &T) -> Value {
        serde_json::to_value(params).expect("params should encode")
    }

    // ── Requests ──

    #[test]
    fn email_find_params_send_the_documented_fields() {
        let params = EmailFindParams {
            fullname: Some("Dwight Schrute".into()),
            company_domain: Some("dundermifflin.com".into()),
            linkedin_url: Some("https://www.linkedin.com/in/dwight-schrute".into()),
            custom: Some(json!({ "crm_id": "lead_001" })),
            settings: Some(SearchSettings {
                retrieve_gender: Some(true),
                retrieve_company_info: Some(true),
                webhook: Some("https://your-app.com/webhooks/enrow".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            encode(&params),
            json!({
                "fullname": "Dwight Schrute",
                "company_domain": "dundermifflin.com",
                "linkedin_url": "https://www.linkedin.com/in/dwight-schrute",
                "custom": { "crm_id": "lead_001" },
                "settings": {
                    "retrieve_gender": true,
                    "retrieve_company_info": true,
                    "webhook": "https://your-app.com/webhooks/enrow"
                }
            })
        );
    }

    #[test]
    fn email_find_bulk_params_send_searches_custom_and_settings() {
        let params = EmailFindBulkParams {
            searches: vec![EmailFindParams {
                fullname: Some("Jim Halpert".into()),
                company_name: Some("Dunder Mifflin".into()),
                custom: Some(json!("lead_002")),
                ..Default::default()
            }],
            custom: Some(json!({ "campaign": "q2" })),
            settings: Some(SearchSettings {
                country_code: Some("US".into()),
                ..Default::default()
            }),
        };

        assert_eq!(
            encode(&params),
            json!({
                "searches": [
                    { "fullname": "Jim Halpert", "company_name": "Dunder Mifflin", "custom": "lead_002" }
                ],
                "custom": { "campaign": "q2" },
                "settings": { "country_code": "US" }
            })
        );
    }

    #[test]
    fn verify_single_params_send_email_custom_and_webhook() {
        let params = VerifySingleParams {
            email: "pam.beesly@dundermifflin.com".into(),
            custom: Some(json!({ "external_id": "do4eIF8jF40x!" })),
            settings: Some(WebhookSettings {
                webhook: Some("https://your-app.com/webhooks/enrow".into()),
            }),
        };

        assert_eq!(
            encode(&params),
            json!({
                "email": "pam.beesly@dundermifflin.com",
                "custom": { "external_id": "do4eIF8jF40x!" },
                "settings": { "webhook": "https://your-app.com/webhooks/enrow" }
            })
        );
    }

    #[test]
    fn verify_bulk_params_send_verifications() {
        let params = VerifyBulkParams {
            emails: vec![
                "pam.beesly@dundermifflin.com".into(),
                "angela.martin@dundermifflin.com".into(),
            ],
            ..Default::default()
        };

        assert_eq!(
            encode(&params),
            json!({
                "verifications": ["pam.beesly@dundermifflin.com", "angela.martin@dundermifflin.com"]
            })
        );
    }

    #[test]
    fn phone_find_params_send_firstname_and_lastname() {
        let params = PhoneFindParams {
            first_name: Some("Michael".into()),
            last_name: Some("Scott".into()),
            company_domain: Some("dundermifflin.com".into()),
            custom: Some(json!("#u84hde941Jdx")),
            ..Default::default()
        };

        assert_eq!(
            encode(&params),
            json!({
                "firstname": "Michael",
                "lastname": "Scott",
                "company_domain": "dundermifflin.com",
                "custom": "#u84hde941Jdx"
            })
        );
    }

    #[test]
    fn phone_find_bulk_params_send_searches_and_settings() {
        let params = PhoneFindBulkParams {
            searches: vec![
                PhoneFindParams {
                    linkedin_url: Some("https://www.linkedin.com/in/michael-scott".into()),
                    ..Default::default()
                },
                PhoneFindParams {
                    first_name: Some("Dwight".into()),
                    last_name: Some("Schrute".into()),
                    company_name: Some("Dunder Mifflin".into()),
                    ..Default::default()
                },
            ],
            settings: Some(WebhookSettings {
                webhook: Some("https://your-app.com/webhooks/enrow".into()),
            }),
            ..Default::default()
        };

        assert_eq!(
            encode(&params),
            json!({
                "searches": [
                    { "linkedin_url": "https://www.linkedin.com/in/michael-scott" },
                    { "firstname": "Dwight", "lastname": "Schrute", "company_name": "Dunder Mifflin" }
                ],
                "settings": { "webhook": "https://your-app.com/webhooks/enrow" }
            })
        );
    }

    // ── Email Finder responses ──

    #[test]
    fn email_find_response() {
        let response: EmailFindResponse = decode(
            r##"{
                "message": "Single search operating",
                "id": "0cf517bc-16e8-45bc-b967-ab9116b3c804",
                "credits_used": 1
            }"##,
        );

        assert_eq!(response.id, "0cf517bc-16e8-45bc-b967-ab9116b3c804");
        assert_eq!(response.message.as_deref(), Some("Single search operating"));
        assert_eq!(response.credits_used, Some(1.0));
    }

    #[test]
    fn email_find_result() {
        let result: EmailFindResult = decode(
            r##"{
                "email": "dwight.schrute@dundermifflin.com",
                "custom": {
                    "external_id": "do4eIF8jF40x!",
                    "partition_number": 2
                },
                "info": {
                    "company_domain": "dundermifflin.com",
                    "fullname": "Dwight Schrute",
                    "firstname": "dwight",
                    "lastname": "schrute"
                },
                "qualification": "valid"
            }"##,
        );

        assert_eq!(result.qualification, "valid");
        assert_eq!(result.email.as_deref(), Some("dwight.schrute@dundermifflin.com"));
        assert_eq!(
            result.custom,
            Some(json!({ "external_id": "do4eIF8jF40x!", "partition_number": 2 }))
        );
        let info = result.info.expect("info");
        assert_eq!(info.company_domain.as_deref(), Some("dundermifflin.com"));
        assert_eq!(info.fullname.as_deref(), Some("Dwight Schrute"));
        assert_eq!(info.first_name.as_deref(), Some("dwight"));
        assert_eq!(info.last_name.as_deref(), Some("schrute"));
        assert!(info.gender.is_none());
        assert!(result.linkedin_status.is_none());
    }

    #[test]
    fn email_find_result_with_every_field() {
        let result: EmailFindResult = decode(
            r##"{
                "email": "dwight.schrute@dundermifflin.com",
                "custom": {"external_id": "do4eIF8jF40x!", "partition_number": 2},
                "info": {
                    "company_domain": "dundermifflin.com",
                    "company_name": "Dunder Mifflin",
                    "fullname": "Dwight Schrute",
                    "firstname": "dwight",
                    "lastname": "schrute",
                    "gender": "male",
                    "companyInfo": {}
                },
                "qualification": "valid",
                "credits_used": 1
            }"##,
        );

        let info = result.info.expect("info");
        assert_eq!(info.company_name.as_deref(), Some("Dunder Mifflin"));
        assert_eq!(info.gender.as_deref(), Some("male"));
        assert_eq!(info.company_info, Some(json!({})));
        assert_eq!(result.credits_used, Some(1.0));
    }

    #[test]
    fn email_find_result_from_a_linkedin_profile() {
        let found: EmailFindResult = decode(
            r##"{
                "email": "dwight.schrute@dundermifflin.com",
                "custom": {
                    "crm_id": "lead_001"
                },
                "info": {
                    "company_domain": "dundermifflin.com",
                    "company_name": "Dunder Mifflin",
                    "firstname": "Dwight",
                    "lastname": "Schrute"
                },
                "linkedin_status": "found",
                "linkedin_url": "https://www.linkedin.com/in/dwight-schrute",
                "qualification": "valid",
                "credits_used": 1
            }"##,
        );

        assert_eq!(found.qualification, "valid");
        assert_eq!(found.linkedin_status.as_deref(), Some("found"));
        assert_eq!(
            found.linkedin_url.as_deref(),
            Some("https://www.linkedin.com/in/dwight-schrute")
        );
        assert_eq!(found.info.expect("info").company_name.as_deref(), Some("Dunder Mifflin"));

        let not_found: EmailFindResult = decode(
            r##"{
                "custom": {
                    "crm_id": "lead_001"
                },
                "linkedin_status": "no_current_company",
                "linkedin_url": "https://www.linkedin.com/in/dwight-schrute",
                "qualification": "invalid",
                "credits_used": 0.25
            }"##,
        );

        assert_eq!(not_found.qualification, "invalid");
        assert_eq!(not_found.linkedin_status.as_deref(), Some("no_current_company"));
        assert_eq!(not_found.credits_used, Some(0.25));
        assert!(not_found.email.is_none());
        assert!(not_found.info.is_none());
    }

    #[test]
    fn email_find_result_still_running() {
        let result: EmailFindResult = decode(r##"{"qualification": "ongoing"}"##);

        assert_eq!(result.qualification, "ongoing");
        assert!(result.email.is_none());
        assert!(result.info.is_none());
    }

    #[test]
    fn email_find_bulk_response() {
        let response: EmailFindBulkResponse = decode(
            r##"{
                "message": "Bulk search operating",
                "id": "0cf517bc-16e8-45bc-b967-ab9116b3c804",
                "credits_used": 3,
                "estimated_duration": 1
            }"##,
        );

        assert_eq!(response.id, "0cf517bc-16e8-45bc-b967-ab9116b3c804");
        assert_eq!(response.credits_used, Some(3.0));
        assert_eq!(response.estimated_duration, Some(1.0));
    }

    #[test]
    fn email_find_bulk_result() {
        let result: EmailFindBulkResult = decode(
            r##"{
                "general": {
                    "id": "3d4cb64e-i64d-41e8-a686-736751004a32",
                    "status": "completed",
                    "custom": "campaign_q2"
                },
                "stats": {
                    "finished": 3,
                    "requested": 3,
                    "valid": 2,
                    "credits_cost": {
                        "initial": 3,
                        "refunded": 1,
                        "final": 2
                    }
                },
                "results": [
                    {
                        "index": "0",
                        "qualification": "valid",
                        "email": "dwight.schrute@dundermifflin.com",
                        "info": {
                            "domain": "dundermifflin.com",
                            "firstname": "Dwight",
                            "lastname": "Schrute",
                            "companyInfo": {
                                "name": "Dunder Mifflin"
                            }
                        },
                        "raw_params": {
                            "company_domain": "dundermifflin.com",
                            "fullname": "Dwight Schrute"
                        },
                        "custom": "lead_001"
                    },
                    {
                        "index": "1",
                        "qualification": "valid",
                        "email": "jim.halpert@dundermifflin.com",
                        "info": {
                            "domain": "dundermifflin.com",
                            "firstname": "Jim",
                            "lastname": "Halpert",
                            "companyInfo": {
                                "name": "Dunder Mifflin"
                            }
                        },
                        "raw_params": {
                            "company_domain": "dundermifflin.com",
                            "fullname": "Jim Halpert"
                        },
                        "custom": "lead_002"
                    },
                    {
                        "index": "2",
                        "qualification": "invalid",
                        "email": null,
                        "raw_params": {
                            "company_domain": "dundermifflin.com",
                            "fullname": "Ryan Howard"
                        },
                        "custom": "lead_003"
                    }
                ]
            }"##,
        );

        assert_eq!(result.general.id, "3d4cb64e-i64d-41e8-a686-736751004a32");
        assert_eq!(result.general.status, "completed");
        assert_eq!(result.general.custom, Some(json!("campaign_q2")));
        assert_eq!(result.stats.finished, Some(3));
        assert_eq!(result.stats.valid, Some(2));
        let credits = result.stats.credits_cost.expect("credits_cost");
        assert_eq!(credits.initial, Some(3.0));
        assert_eq!(credits.refunded, Some(1.0));
        assert_eq!(credits.final_cost, Some(2.0));

        let results = result.results.expect("results");
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].index.as_deref(), Some("0"));
        assert_eq!(results[0].qualification, "valid");
        let info = results[0].info.as_ref().expect("info");
        assert_eq!(info.domain.as_deref(), Some("dundermifflin.com"));
        assert_eq!(info.first_name.as_deref(), Some("Dwight"));
        assert_eq!(info.company_info, Some(json!({ "name": "Dunder Mifflin" })));
        assert_eq!(
            results[0].raw_params,
            Some(json!({ "company_domain": "dundermifflin.com", "fullname": "Dwight Schrute" }))
        );
        assert_eq!(results[0].custom, Some(json!("lead_001")));
        assert_eq!(results[2].qualification, "invalid");
        assert!(results[2].email.is_none());
        assert!(results[2].info.is_none());
    }

    #[test]
    fn email_find_bulk_result_still_running() {
        let result: EmailFindBulkResult = decode(
            r##"{"general": {"id": "3d4cb64e-i64d-41e8-a686-736751004a32", "status": "ongoing"}, "stats": {"requested": 3, "finished": 2}}"##,
        );

        assert_eq!(result.general.status, "ongoing");
        assert_eq!(result.stats.finished, Some(2));
        assert_eq!(result.stats.requested, Some(3));
        assert!(result.stats.credits_cost.is_none());
        assert!(result.results.is_none());
    }

    // ── Email Verifier responses ──

    #[test]
    fn verify_single_response() {
        let response: VerifySingleResponse = decode(
            r##"{
                "message": "Single verification operating",
                "id": "0cf517bc-16e8-45bc-b967-ab9116b3c804",
                "credits_used": 0.25,
                "credits": {
                    "amount": 0.25,
                    "source": "sub",
                    "split": { "fromPaygo": 0, "fromSub": 0.25 }
                }
            }"##,
        );

        assert_eq!(response.id, "0cf517bc-16e8-45bc-b967-ab9116b3c804");
        assert_eq!(response.credits_used, Some(0.25));
        let credits = response.credits.expect("credits");
        assert_eq!(credits.amount, Some(0.25));
        assert_eq!(credits.source.as_deref(), Some("sub"));
        let split = credits.split.expect("split");
        assert_eq!(split.from_sub, Some(0.25));
        assert_eq!(split.from_paygo, Some(0.0));
    }

    #[test]
    fn verify_single_result() {
        let result: VerifySingleResult = decode(
            r##"{
                "email": "pam.beesly@dundermifflin.com",
                "custom": {
                    "external_id": "do4eIF8jF40x!"
                },
                "qualification": "valid"
            }"##,
        );

        assert_eq!(result.qualification, "valid");
        assert_eq!(result.email.as_deref(), Some("pam.beesly@dundermifflin.com"));
        assert_eq!(result.custom, Some(json!({ "external_id": "do4eIF8jF40x!" })));

        let ongoing: VerifySingleResult = decode(r##"{"qualification": "ongoing"}"##);
        assert_eq!(ongoing.qualification, "ongoing");
        assert!(ongoing.email.is_none());
    }

    #[test]
    fn verify_bulk_response() {
        let response: VerifyBulkResponse = decode(
            r##"{
                "message": "Bulk verification operating",
                "id": "0cf517bc-16e8-45bc-b967-ab9116b3c804",
                "credits_used": 0.75,
                "estimated_duration": 1
            }"##,
        );

        assert_eq!(response.id, "0cf517bc-16e8-45bc-b967-ab9116b3c804");
        assert_eq!(response.credits_used, Some(0.75));
        assert_eq!(response.estimated_duration, Some(1.0));
        assert!(response.credits.is_none());
    }

    #[test]
    fn verify_bulk_result() {
        let result: VerifyBulkResult = decode(
            r##"{
                "general": {
                    "id": "9d4fa0ed-f76b-409f-bee4-d5468ebda70e",
                    "status": "completed"
                },
                "results": [
                    {
                        "email": "pam.beesly@dundermifflin.com",
                        "index": "195f87c6-376e-4eca-9861-f92935413b0c",
                        "qualification": "valid"
                    },
                    {
                        "email": "angela.martin@dundermifflin.com",
                        "index": "662784ae-3722-47af-b056-7554e013f73b",
                        "qualification": "valid"
                    }
                ],
                "stats": {
                    "credits_cost": 0.5,
                    "requested": 2,
                    "valid": 2
                }
            }"##,
        );

        assert_eq!(result.general.id, "9d4fa0ed-f76b-409f-bee4-d5468ebda70e");
        assert_eq!(result.general.status, "completed");
        assert!(result.general.custom.is_none());
        assert_eq!(result.stats.credits_cost, Some(0.5));
        assert_eq!(result.stats.requested, Some(2));
        assert_eq!(result.stats.valid, Some(2));
        let results = result.results.expect("results");
        assert_eq!(results.len(), 2);
        assert_eq!(results[1].email.as_deref(), Some("angela.martin@dundermifflin.com"));
        assert_eq!(
            results[1].index.as_deref(),
            Some("662784ae-3722-47af-b056-7554e013f73b")
        );
        assert_eq!(results[1].qualification, "valid");
    }

    #[test]
    fn verify_bulk_result_still_running() {
        let result: VerifyBulkResult = decode(
            r##"{"general": {"id": "9d4fa0ed-f76b-409f-bee4-d5468ebda70e", "status": "ongoing"}, "stats": {"finished": 1, "requested": 2}}"##,
        );

        assert_eq!(result.general.status, "ongoing");
        assert_eq!(result.stats.finished, Some(1));
        assert!(result.stats.valid.is_none());
        assert!(result.results.is_none());
    }

    // ── Phone Finder responses ──

    #[test]
    fn phone_find_response() {
        let response: PhoneFindResponse = decode(
            r##"{
                "credits_used": 40,
                "id": "94cbc13b-7d77-4f41-83f0-60106daa7182",
                "message": "Single search operating",
                "credits": {
                    "amount": 40,
                    "source": "sub",
                    "split": { "fromPaygo": 0, "fromSub": 40 }
                }
            }"##,
        );

        assert_eq!(response.id, "94cbc13b-7d77-4f41-83f0-60106daa7182");
        assert_eq!(response.credits_used, Some(40.0));
        let credits = response.credits.expect("credits");
        assert_eq!(credits.source.as_deref(), Some("sub"));
        assert_eq!(credits.split.expect("split").from_sub, Some(40.0));
    }

    #[test]
    fn phone_find_result() {
        let found: PhoneFindResult = decode(
            r##"{
                "params": {
                    "linkedin_url": "https://www.linkedin.com/in/michael-scott/"
                },
                "custom": "#u84hde941Jdx",
                "number": "+15705551234",
                "country": "US",
                "qualification": "found"
            }"##,
        );

        assert_eq!(found.qualification, "found");
        assert_eq!(found.number.as_deref(), Some("+15705551234"));
        assert_eq!(found.country.as_deref(), Some("US"));
        assert_eq!(
            found.params,
            Some(json!({ "linkedin_url": "https://www.linkedin.com/in/michael-scott/" }))
        );
        assert_eq!(found.custom, Some(json!("#u84hde941Jdx")));

        let not_found: PhoneFindResult = decode(r##"{"qualification": "not_found"}"##);
        assert_eq!(not_found.qualification, "not_found");
        assert!(not_found.number.is_none());

        let ongoing: PhoneFindResult = decode(r##"{"qualification": "ongoing"}"##);
        assert_eq!(ongoing.qualification, "ongoing");
    }

    #[test]
    fn phone_bulk_response() {
        let response: PhoneBulkResponse = decode(
            r##"{
                "credits_used": 80,
                "estimated_duration": 1,
                "id": "e3b61122-d6a6-4ea7-b331-9b734682a76a",
                "message": "Bulk search operating",
                "credits": {
                    "amount": 80,
                    "source": "sub",
                    "split": {
                        "fromPaygo": 0,
                        "fromSub": 80
                    }
                }
            }"##,
        );

        assert_eq!(response.id, "e3b61122-d6a6-4ea7-b331-9b734682a76a");
        assert_eq!(response.credits_used, Some(80.0));
        assert_eq!(response.estimated_duration, Some(1.0));
        assert_eq!(response.credits.expect("credits").amount, Some(80.0));
    }

    #[test]
    fn phone_bulk_result() {
        let result: PhoneBulkResult = decode(
            r##"{
                "general": {
                    "id": "e3b61122-d6a6-4ea7-b331-9b734682a76a",
                    "status": "completed"
                },
                "results": [
                    {
                        "country": "US",
                        "index": 0,
                        "number": "+15705551234",
                        "params": {
                            "linkedin_url": "https://www.linkedin.com/in/michael-scott"
                        },
                        "qualification": "found"
                    },
                    {
                        "country": "US",
                        "index": 1,
                        "number": "+15705555678",
                        "params": {
                            "linkedin_url": "https://www.linkedin.com/in/dwight-schrute"
                        },
                        "qualification": "found"
                    }
                ],
                "stats": {
                    "credits_cost": 80,
                    "credits_to_refund": 0,
                    "found": 2,
                    "requested": 2
                }
            }"##,
        );

        assert_eq!(result.general.status, "completed");
        assert_eq!(result.stats.credits_cost, Some(80.0));
        assert_eq!(result.stats.credits_to_refund, Some(0.0));
        assert_eq!(result.stats.found, Some(2));
        assert_eq!(result.stats.requested, Some(2));
        let results = result.results.expect("results");
        assert_eq!(results.len(), 2);
        assert_eq!(results[1].index, Some(1));
        assert_eq!(results[1].number.as_deref(), Some("+15705555678"));
        assert_eq!(results[1].country.as_deref(), Some("US"));
        assert_eq!(results[1].qualification, "found");
        assert_eq!(
            results[1].params,
            Some(json!({ "linkedin_url": "https://www.linkedin.com/in/dwight-schrute" }))
        );
    }

    #[test]
    fn phone_bulk_result_still_running() {
        let result: PhoneBulkResult = decode(
            r##"{"general": {"id": "e3b61122-d6a6-4ea7-b331-9b734682a76a", "status": "ongoing"}, "stats": {"finished": 0, "requested": 1}}"##,
        );

        assert_eq!(result.general.status, "ongoing");
        assert_eq!(result.stats.finished, Some(0));
        assert!(result.stats.found.is_none());
        assert!(result.results.is_none());
    }

    // ── Account ──

    #[test]
    fn account_info() {
        let info: AccountInfo = decode(
            r##"{
                "credits": 36000,
                "webhooks": [
                    "https://api.mysuperwebsite/api/webhook"
                ]
            }"##,
        );

        assert_eq!(info.credits, 36000.0);
        assert_eq!(info.webhooks, vec!["https://api.mysuperwebsite/api/webhook"]);

        // `webhooks` may be absent, and fields the docs don't list are ignored.
        let info: AccountInfo = decode(
            r##"{"credits": 8500.75, "credits_breakdown": {"paygo": 500, "subscription": 8000.75}}"##,
        );
        assert_eq!(info.credits, 8500.75);
        assert!(info.webhooks.is_empty());
    }
}
