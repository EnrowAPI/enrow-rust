# Enrow Rust SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/EnrowAPI/enrow-rust)](https://github.com/EnrowAPI/enrow-rust)
[![Last commit](https://img.shields.io/github/last-commit/EnrowAPI/enrow-rust)](https://github.com/EnrowAPI/enrow-rust/commits)

Find and verify professional emails and phone numbers with the [Enrow API](https://enrow.io).

## Install

The crate is not published on crates.io yet: add it to your `Cargo.toml` as a git dependency. The examples run on `tokio` with its `full` feature, and build `custom` values with `serde_json`.

```toml
[dependencies]
enrow = { git = "https://github.com/EnrowAPI/enrow-rust" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## Quick start

```rust
use enrow::{EmailFindParams, Enrow};

#[tokio::main]
async fn main() -> enrow::Result<()> {
    let client = Enrow::new("YOUR_API_KEY");

    let result = client
        .email
        .find_and_wait(
            &EmailFindParams {
                fullname: Some("Dwight Schrute".into()),
                company_domain: Some("dundermifflin.com".into()),
                ..Default::default()
            },
            None,
        )
        .await?;

    println!("{} {:?}", result.qualification, result.email);
    Ok(())
}
```

Every search is asynchronous. `find`, `single`, `find_bulk` and `bulk` launch it and return its `id`; `get` and `get_bulk` read the result, which stays `ongoing` until the search ends. `find_and_wait` does both and polls for you (every 2 seconds for up to 30 seconds by default, see `PollOptions`). To skip polling, pass a `webhook` URL in `settings`: Enrow calls it when the search ends, with the result of a single search.

The snippets below run inside an `async` function that returns `enrow::Result<()>`, with the `client` above.

## Email Finder

```rust
use enrow::{EmailFindBulkParams, EmailFindParams, SearchSettings};
use serde_json::json;

// Launch a search, then read its result with its id
let search = client.email.find(&EmailFindParams {
    fullname: Some("Dwight Schrute".into()),
    company_name: Some("Dunder Mifflin".into()),
    custom: Some(json!({ "crm_id": "lead_001" })),
    settings: Some(SearchSettings {
        country_code: Some("US".into()),
        retrieve_gender: Some(true),
        ..Default::default()
    }),
    ..Default::default()
}).await?;
let result = client.email.get(&search.id).await?;
if result.qualification == "valid" {
    println!("{:?} {:?}", result.email, result.info);
}

// Search from a LinkedIn profile URL
let result = client.email.find_and_wait(&EmailFindParams {
    linkedin_url: Some("https://www.linkedin.com/in/dwight-schrute".into()),
    ..Default::default()
}, None).await?;
println!("{:?} {:?}", result.linkedin_status, result.email);

// Bulk: up to 5,000 searches
let batch = client.email.find_bulk(&EmailFindBulkParams {
    searches: vec![
        EmailFindParams {
            fullname: Some("Dwight Schrute".into()),
            company_domain: Some("dundermifflin.com".into()),
            custom: Some(json!("lead_001")),
            ..Default::default()
        },
        EmailFindParams {
            fullname: Some("Jim Halpert".into()),
            company_domain: Some("dundermifflin.com".into()),
            ..Default::default()
        },
    ],
    ..Default::default()
}).await?;
let batch = client.email.get_bulk(&batch.id).await?;
if batch.general.status == "completed" {
    for row in batch.results.unwrap_or_default() {
        println!("{:?} {} {:?}", row.index, row.qualification, row.email);
    }
}
```

## Email Verifier

```rust
use enrow::{VerifyBulkParams, VerifySingleParams};

let verification = client.verify.single(&VerifySingleParams {
    email: "pam.beesly@dundermifflin.com".into(),
    ..Default::default()
}).await?;
let result = client.verify.get(&verification.id).await?;
println!("{}", result.qualification); // "valid", "invalid", or "ongoing" while it runs

// Bulk: up to 5,000 addresses (sent as `verifications`)
let batch = client.verify.bulk(&VerifyBulkParams {
    emails: vec![
        "pam.beesly@dundermifflin.com".into(),
        "angela.martin@dundermifflin.com".into(),
    ],
    ..Default::default()
}).await?;
let batch = client.verify.get_bulk(&batch.id).await?;
```

## Phone Finder

```rust
use enrow::{PhoneFindBulkParams, PhoneFindParams};

// From a LinkedIn profile URL (recommended)
let result = client.phone.find_and_wait(&PhoneFindParams {
    linkedin_url: Some("https://www.linkedin.com/in/michael-scott/".into()),
    ..Default::default()
}, None).await?;
println!("{} {:?} {:?}", result.qualification, result.number, result.country); // "found" or "not_found"

// From a first name, a last name and a company
let search = client.phone.find(&PhoneFindParams {
    first_name: Some("Michael".into()),
    last_name: Some("Scott".into()),
    company_domain: Some("dundermifflin.com".into()),
    ..Default::default()
}).await?;
let result = client.phone.get(&search.id).await?;

// Bulk: up to 3,000 searches
let batch = client.phone.find_bulk(&PhoneFindBulkParams {
    searches: vec![PhoneFindParams {
        linkedin_url: Some("https://www.linkedin.com/in/dwight-schrute".into()),
        ..Default::default()
    }],
    ..Default::default()
}).await?;
let batch = client.phone.get_bulk(&batch.id).await?;
```

## Account

```rust
let info = client.account.info().await?;
println!("Credits: {}", info.credits);
```

## Error handling

```rust
use enrow::{EmailFindParams, EnrowError};

let params = EmailFindParams {
    fullname: Some("Dwight Schrute".into()),
    company_domain: Some("dundermifflin.com".into()),
    ..Default::default()
};

match client.email.find(&params).await {
    Ok(search) => println!("Search launched: {}", search.id),
    Err(EnrowError::InsufficientBalance { message }) => eprintln!("Not enough credits (402): {}", message),
    Err(EnrowError::RateLimit { message }) => eprintln!("Rate limited (429): {}", message),
    Err(EnrowError::Authentication { message }) => eprintln!("API key refused (401): {}", message),
    Err(EnrowError::Api { status, message, .. }) => eprintln!("Error {}: {}", status, message),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Credits

| Endpoint | Cost |
|----------|------|
| Email Finder | 1 credit per email found |
| Email Verifier | 0.25 credit per check |
| Phone Finder | 40 credits per phone found (50 on legacy plans) |

A search launched from a LinkedIn profile URL costs 0.25 credit when no email is found. See [Credits & billing](https://docs.enrow.io/credits-billing).

## Links

- [API Documentation](https://docs.enrow.io)
- [Enrow](https://enrow.io)

## License

MIT
