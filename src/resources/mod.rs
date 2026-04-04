mod email_finder;
mod email_verifier;
mod phone_finder;
mod reverse_email;
mod account;

pub use email_finder::EmailFinder;
pub use email_verifier::EmailVerifier;
pub use phone_finder::PhoneFinder;
pub use reverse_email::ReverseEmailResource;
pub use account::AccountResource;

use std::future::Future;
use crate::error::Result;
use crate::types::PollOptions;

pub(crate) async fn poll<T, F, Fut>(
    fetcher: F,
    is_done: fn(&T) -> bool,
    opts: PollOptions,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let start = tokio::time::Instant::now();
    loop {
        let result = fetcher().await?;
        if is_done(&result) {
            return Ok(result);
        }
        if start.elapsed() >= opts.timeout {
            return Ok(result);
        }
        tokio::time::sleep(opts.poll_interval).await;
    }
}
