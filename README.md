# Enrow Rust SDK

Find and verify professional emails, phone numbers, and contact information with the [Enrow API](https://enrow.io).

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
enrow = "1"
tokio = { version = "1", features = ["full"] }
```

## Quick start

```rust
use enrow::{Enrow, EmailFindParams};

#[tokio::main]
async fn main() {
    let client = Enrow::new("your_api_key");

    let result = client.email.find(&EmailFindParams {
        company_domain: Some("apple.com".into()),
        first_name: Some("Tim".into()),
        last_name: Some("Cook".into()),
        ..Default::default()
    }).await.unwrap();

    println!("{:?}", result.email);
}
```

## Email Finder

```rust
// Find with auto-polling
let result = client.email.find_and_wait(&params, None).await?;

// Get by ID
let result = client.email.get("search_abc123").await?;

// Bulk
let batch = client.email.find_bulk(&EmailFindBulkParams {
    searches: vec![params1, params2],
    settings: None,
}).await?;
let results = client.email.get_bulk(&batch.batch_id).await?;
```

## Email Verifier

```rust
let result = client.verify.single(&VerifySingleParams {
    email: "tcook@apple.com".into(),
    settings: None,
}).await?;

let batch = client.verify.bulk(&VerifyBulkParams {
    emails: vec!["a@b.com".into(), "c@d.com".into()],
    settings: None,
}).await?;
```

## Phone Finder

```rust
let result = client.phone.find_and_wait(&PhoneFindParams {
    linkedin_url: Some("https://linkedin.com/in/timcook".into()),
    ..Default::default()
}, None).await?;
```

## Reverse Email

```rust
let result = client.reverse_email.find(&ReverseEmailParams {
    email: "tcook@apple.com".into(),
    settings: None,
}).await?;
```

## Account

```rust
let info = client.account.info().await?;
println!("Credits: {}", info.credits);
```

## Error handling

```rust
use enrow::EnrowError;

match client.email.find(&params).await {
    Ok(result) => println!("{:?}", result),
    Err(EnrowError::RateLimit { message }) => eprintln!("Rate limited: {}", message),
    Err(EnrowError::InsufficientBalance { message }) => eprintln!("No credits: {}", message),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Credits

| Endpoint | Cost |
|----------|------|
| Email Finder | 1 credit/email |
| Email Verifier | 0.25 credit/email |
| Phone Finder | 50 credits/phone |
| Reverse Email | 5 credits/lookup |

## Links

- [API Documentation](https://docs.enrow.io)
- [Enrow](https://enrow.io)

## License

MIT
