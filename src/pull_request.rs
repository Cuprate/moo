//! TODO

//---------------------------------------------------------------------------------------------------- Use

use matrix_sdk::ruma::OwnedUserId;
use serde::{Deserialize, Serialize};

use crate::priority::Priority;

//----------------------------------------------------------------------------------------------------
/// TODO
pub type PullRequest = u64;

//----------------------------------------------------------------------------------------------------
/// TODO
#[derive(Debug, thiserror::Error)]
pub enum PullRequestError {
    /// TODO
    #[error("#{0} does not exist")]
    DoesNotExist(PullRequest),

    /// TODO
    #[error("#{0} is not a pull request")]
    IsNotPullRequest(PullRequest),

    /// TODO
    #[error("#{0} is already merged")]
    AlreadyMerged(PullRequest),

    /// TODO
    #[error("#{pr} error: {error}")]
    Other {
        /// TODO
        pr: PullRequest,
        /// TODO
        error: anyhow::Error,
    },
}

impl PullRequestError {
    /// TODO
    pub const fn other(pr: PullRequest, error: anyhow::Error) -> Self {
        Self::Other { pr, error }
    }
}

//----------------------------------------------------------------------------------------------------
/// TODO
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PullRequestMetadata {
    /// Priority of the pull request.
    pub priority: Priority,

    /// The user who submit the pull
    /// request to the merge queue.
    ///
    /// This is _not_ the creator of the PR.
    pub user: OwnedUserId,

    /// UNIX timestamp of when this PR
    /// was added to the merge queue.
    ///
    /// This is _not_ the timestamp of
    /// when the PR was created.
    pub timestamp: u64,
}
