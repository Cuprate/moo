//! TODO

use std::num::NonZero;

//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tracing::{info, instrument, trace};

use crate::{
    constants::{
        CONFIG, CUPRATE_GITHUB_PULL_API, CUPRATE_MEETING_WEEKDAY, MONERO_META_GITHUB_ISSUE,
        MONERO_META_GITHUB_ISSUE_API, MOO_GITHUB_ID, MOO_USER_AGENT, TXT_CUPRATE_MEETING_PREFIX,
        TXT_CUPRATE_MEETING_SUFFIX,
    },
    pull_request::{PullRequest, PullRequestError},
};

//---------------------------------------------------------------------------------------------------- Free
/// TODO
fn build_client() -> Client {
    reqwest::ClientBuilder::new()
        .gzip(true)
        .user_agent(MOO_USER_AGENT)
        .build()
        .expect("this can't error")
}

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

    let client = build_client();

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

//---------------------------------------------------------------------------------------------------- Issues
/// TODO
trait AddGithubHeaders {
    /// TODO
    fn add_github_headers(self) -> Self;
}

impl AddGithubHeaders for RequestBuilder {
    fn add_github_headers(self) -> Self {
        self.header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Authorization", format!("Bearer {}", CONFIG.token))
    }
}

/// TODO
///
/// # Errors
/// TODO
pub async fn current_meeting_url() -> Result<String, anyhow::Error> {
    let client = build_client();
    let (issue, _) = find_cuprate_meeting_issue(&client, false).await?;
    Ok(format!("{MONERO_META_GITHUB_ISSUE}/{issue}"))
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn finish_cuprate_meeting(
    meeting_logs: String,
) -> Result<(String, String), anyhow::Error> {
    let client = build_client();

    let (issue, title) = find_cuprate_meeting_issue(&client, false).await?;
    let logs = post_comment_in_issue(&client, issue, &meeting_logs).await?;
    let next_meeting = post_cuprate_meeting_issue(&client, title, issue, None).await?;
    close_issue(&client, issue).await?;

    Ok((logs, next_meeting))
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn cancel_cuprate_meeting(
    count: NonZero<u8>,
    reason: Option<&str>,
) -> Result<(Vec<String>, String), anyhow::Error> {
    let client = build_client();
    let reason = reason.unwrap_or("Unknown");
    let comment = format!("This meeting was canceled, reason: `{reason}`");

    let mut canceled_meetings = vec![];
    let mut next_meeting = String::new();

    for week_multiplier in 1..=count.get() {
        let (issue, title) = find_cuprate_meeting_issue(&client, false).await?;

        post_comment_in_issue(&client, issue, &comment).await?;

        let next = post_cuprate_meeting_issue(
            &client,
            title,
            issue,
            NonZero::new(u64::from(week_multiplier)),
        )
        .await?;

        close_issue(&client, issue).await?;

        canceled_meetings.push(format!("{MONERO_META_GITHUB_ISSUE}/{issue}"));
        next_meeting = next;
    }

    Ok((canceled_meetings, next_meeting))
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn find_cuprate_meeting_issue(
    client: &Client,
    find_last_issue: bool,
) -> Result<(u64, String), anyhow::Error> {
    trace!("Finding Cuprate meeting issue on: {MONERO_META_GITHUB_ISSUE_API}");

    let body = client
        .get(MONERO_META_GITHUB_ISSUE_API)
        .add_github_headers()
        .query(&[("state", "all")])
        .send()
        .await?
        .text()
        .await?;

    trace!("reply: {body}");

    /// TODO
    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        /// TODO
        number: u64,
        /// TODO
        title: String,
        /// TODO
        user: User,
    }

    /// TODO
    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        /// TODO
        login: String,
    }

    let responses = from_str::<Vec<Response>>(&body)?;
    trace!("responses: {responses:#?}");

    let mut second = false;
    for resp in responses {
        if resp.user.login == MOO_GITHUB_ID && resp.title.contains("Cuprate Meeting") {
            if find_last_issue {
                if second {
                    return Ok((resp.number, resp.title));
                }
                second = true;
                continue;
            }

            return Ok((resp.number, resp.title));
        }
    }

    Err(anyhow!("Error: couldn't find Cuprate Meeting issue"))
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn post_cuprate_meeting_issue(
    client: &Client,
    previous_meeting_title: String,
    last_issue: u64,
    week_multiplier: Option<NonZero<u64>>,
) -> Result<String, anyhow::Error> {
    trace!("Posting Cuprate meeting issue on: {MONERO_META_GITHUB_ISSUE_API}");

    let next_meeting_iso_8601 = {
        use chrono::{prelude::*, Days};
        let mut today = Utc::now().date_naive();

        while today.weekday() != CUPRATE_MEETING_WEEKDAY {
            today = today - Days::new(1);
        }

        let next = if let Some(multiplier) = week_multiplier {
            today + Days::new(7 * multiplier.get())
        } else {
            today + Days::new(7)
        };

        next.format("%Y-%m-%d").to_string()
    };

    info!("Next meeting date: {next_meeting_iso_8601}");

    let next_meeting_number = {
        let mut iter = previous_meeting_title.split_whitespace();

        let err = || anyhow!("Failed to parse previous meeting title: {previous_meeting_title}");

        if iter.next().is_none_or(|s| s != "Cuprate") {
            return Err(err());
        }

        if iter.next().is_none_or(|s| s != "Meeting") {
            return Err(err());
        }

        let Some(number_with_hash) = iter.next() else {
            return Err(err());
        };

        let Some(number) = number_with_hash.get(1..) else {
            return Err(err());
        };

        let Ok(number) = number.parse::<u64>() else {
            return Err(err());
        };

        number + 1
    };

    let title = format!(
        "Cuprate Meeting #{next_meeting_number} - Tuesday, {next_meeting_iso_8601}, UTC 18:00"
    );

    info!("Meeting title: {title}");

    let body = format!("{TXT_CUPRATE_MEETING_PREFIX}\n{TXT_CUPRATE_MEETING_SUFFIX}\n\nPrevious meeting: #{last_issue}");

    let body = json!({
        "title": title,
        "body": body,
    });

    info!("Posting issue: {body}");

    let body = client
        .post(MONERO_META_GITHUB_ISSUE_API)
        .add_github_headers()
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    trace!("reply: {body}");

    /// TODO
    #[derive(Serialize, Deserialize)]
    struct Response {
        /// TODO
        html_url: String,
    }

    match from_str::<Response>(&body) {
        Ok(resp) => Ok(resp.html_url),
        Err(e) => Err(anyhow!("Posting issue error: {e}")),
    }
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn post_comment_in_issue(
    client: &Client,
    issue: u64,
    comment: &str,
) -> Result<String, anyhow::Error> {
    let url = format!("{MONERO_META_GITHUB_ISSUE_API}/{issue}/comments");

    trace!("Posting comment on: {url}");

    let body = client
        .post(url)
        .add_github_headers()
        .body(json!({"body":comment}).to_string())
        .send()
        .await?
        .text()
        .await?;

    trace!("Issue comment: {body}");

    /// TODO
    #[derive(Serialize, Deserialize)]
    struct Response {
        /// TODO
        html_url: String,
    }

    match from_str::<Response>(&body) {
        Ok(resp) => {
            if resp.html_url.is_empty() {
                Err(anyhow!("Issue comment error: {body:#?}"))
            } else {
                Ok(resp.html_url)
            }
        }
        Err(e) => return Err(anyhow!("Issue comment error: {e}")),
    }
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn close_issue(client: &Client, issue: u64) -> Result<(), anyhow::Error> {
    let url = format!("{MONERO_META_GITHUB_ISSUE_API}/{issue}");

    trace!("Closing issue: {url}");

    let body = json!({
        "state": "closed"
    });

    let body = client
        .patch(url)
        .add_github_headers()
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    trace!("reply: {body}");

    /// TODO
    #[derive(Serialize, Deserialize)]
    struct Response {
        /// TODO
        number: u64,
        /// TODO
        state: String,
    }

    match from_str::<Response>(&body) {
        Ok(resp) => {
            if resp.state == "closed" {
                Ok(())
            } else {
                Err(anyhow!("Issue close error: {body}"))
            }
        }
        Err(e) => Err(anyhow!("Issue close error: {e}")),
    }
}

/// TODO
///
/// # Errors
/// TODO
#[instrument]
#[inline]
pub async fn edit_cuprate_meeting_agenda(new_items: Vec<String>) -> Result<String, anyhow::Error> {
    let client = build_client();

    let current_issue = find_cuprate_meeting_issue(&client, false).await?.0;
    let last_issue = find_cuprate_meeting_issue(&client, true).await?.0;

    let url = format!("{MONERO_META_GITHUB_ISSUE_API}/{current_issue}");

    trace!("Editing Cuprate meeting agenda on: {url}");

    let new_agenda = {
        let mut buf = String::new();

        for item in new_items {
            buf += "- ";
            buf += item.trim();
            buf += "\n";
        }

        buf
    };

    info!("New meeting agenda: {new_agenda}");

    let body = json!({
        "body": format!("{TXT_CUPRATE_MEETING_PREFIX}\n{new_agenda}\n{TXT_CUPRATE_MEETING_SUFFIX}\n\nPrevious meeting: #{last_issue}"),
    });

    info!("New meeting agenda: {body}");

    let body = client
        .patch(&url)
        .add_github_headers()
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    trace!("reply: {body}");

    /// TODO
    #[derive(Serialize, Deserialize)]
    struct Response {
        /// TODO
        number: u64,
        /// TODO
        html_url: String,
    }

    match from_str::<Response>(&body) {
        Ok(resp) => Ok(resp.html_url),
        Err(e) => Err(anyhow!("Issue edit error: {e}")),
    }
}
