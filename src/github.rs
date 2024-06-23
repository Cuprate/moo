//! TODO

//---------------------------------------------------------------------------------------------------- Use

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tracing::{instrument, trace};

use crate::{
    constants::{CUPRATE_GITHUB_PULL_API, MOO_USER_AGENT},
    pull_request::{PullRequest, PullRequestError},
};

//---------------------------------------------------------------------------------------------------- Event
/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn pr_is_open(pr: PullRequest) -> Result<bool, PullRequestError> {
    let url = format!("{CUPRATE_GITHUB_PULL_API}/{pr}");
    trace!("PR url: {url}");

    let client = match reqwest::ClientBuilder::new()
        .gzip(true)
        .user_agent(MOO_USER_AGENT)
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(PullRequestError::Other {
                pr,
                error: e.into(),
            })
        }
    };

    let req = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => return Err(PullRequestError::other(pr, anyhow!("{e}"))),
    };

    let body = match req.text().await {
        Ok(b) => b,
        Err(e) => return Err(PullRequestError::other(pr, anyhow!("{e}"))),
    };

    trace!("PR url body: {body}");

    /// TODO
    #[derive(Serialize, Deserialize)]
    struct Response {
        /// TODO
        state: Option<String>,
        /// TODO
        status: Option<String>,
    }

    let response: Response = match from_str(&body) {
        Ok(b) => b,
        Err(e) => return Err(PullRequestError::other(pr, anyhow!("{e}"))),
    };

    if response.status.is_some_and(|s| s == "404") {
        return Err(PullRequestError::DoesNotExist(pr));
    }

    let Some(state) = response.state else {
        return Err(PullRequestError::DoesNotExist(pr));
    };

    match state.as_str() {
        "open" => Ok(true),
        "closed" => Ok(false),
        "404" => Err(PullRequestError::DoesNotExist(pr)),
        _ => Err(PullRequestError::other(
            pr,
            anyhow!("failed to parse GitHub response"),
        )),
    }
}
