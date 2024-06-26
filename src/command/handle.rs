//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{
    sync::{atomic::Ordering, Arc},
    time::{SystemTime, UNIX_EPOCH},
};

use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, OwnedUserId};
use readable::up::{Uptime, UptimeFull};
use tracing::{info, instrument, trace};

use crate::{
    command::Command,
    constants::{
        CONFIG, CUPRATE_GITHUB_PULL, CUPRATE_MEETING_UTC_HOUR, CUPRATE_MEETING_WEEKDAY, HELP,
        INIT_INSTANT, MOO, TXT_EMPTY, TXT_MEETING_START_IDENT,
    },
    database::Database,
    github::pr_is_open,
    meeting::{MEETING_DATABASE, MEETING_ONGOING},
    priority::Priority,
    pull_request::{PullRequest, PullRequestMetadata},
};

//---------------------------------------------------------------------------------------------------- Macros
/// TODO
macro_rules! return_if_empty {
    ($db:ident) => {
        if $db.is_empty() {
            info!(TXT_EMPTY);
            return RoomMessageEventContent::text_plain(TXT_EMPTY);
        }
    };
}

//---------------------------------------------------------------------------------------------------- Command
impl Command {
    /// TODO
    ///
    /// # Errors
    /// TODO
    #[allow(clippy::missing_panics_doc)] // rwlock
    #[instrument]
    pub async fn handle(
        self,
        db: &Arc<Database>,
        user: OwnedUserId,
        timestamp: u64,
    ) -> Result<(), anyhow::Error> {
        let db = Arc::clone(db);

        let msg = tokio::task::spawn_blocking(move || async move {
            match self {
                Self::Queue => Self::handle_queue(db).await,
                Self::List => Self::handle_list(db).await,
                Self::Json => Self::handle_json(db).await,
                Self::Add((prs, priority)) => {
                    Self::handle_add(db, prs, priority, user, timestamp).await
                }
                Self::Remove(prs) => Self::handle_remove(db, prs).await,
                Self::Sweep => Self::handle_sweep(db).await,
                Self::Sweeper => Self::handle_sweeper(db).await,
                Self::Clear => Self::handle_clear(db).await,
                Self::Meeting => Self::handle_meeting().await,
                Self::Agenda(items) => Self::handle_agenda(items).await,
                Self::Status => Self::handle_status(),
                Self::Help => Self::handle_help(),
                Self::Shutdown => Self::handle_shutdown(db).await,
            }
        })
        .await?;

        let msg = msg.await;
        crate::free::send(msg).await?;
        Ok(())
    }

    /// TODO
    #[instrument]
    async fn handle_queue(db: Arc<Database>) -> RoomMessageEventContent {
        let db = db.read().await;
        return_if_empty!(db);

        let unix_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut critical = vec![];
        let mut high = vec![];
        let mut medium = vec![];
        let mut low = vec![];

        // Sort each PR by priority level, retain their time in the merge queue.
        for (pr, metadata) in db.iter() {
            let vec = match metadata.priority {
                Priority::Critical => &mut critical,
                Priority::High => &mut high,
                Priority::Medium => &mut medium,
                Priority::Low => &mut low,
            };
            let added = Uptime::from(unix_now.saturating_sub(metadata.timestamp));
            vec.push((pr, added));
        }

        // The final message string `moo` will report.
        let mut msg = String::new();

        // Add each PR in this sorting order:
        //
        // 1. Priority
        // 2. Time in merge queue
        for (mut vec, header) in [
            (critical, "Critical"),
            (high, "High"),
            (medium, "Medium"),
            (low, "Low"),
        ] {
            if vec.is_empty() {
                continue;
            }

            // Add priority header.
            msg.push_str("##### ");
            msg.push_str(header);
            // Add amount of PRs
            msg.push_str(&format!(" ({})", vec.len()));

            // Within each priority level, sort the
            // PRs by how long they have been in
            // the merge queue.
            vec.sort_by(|a, b| b.1.cmp(&a.1));

            // Add each PR, highest priority first.
            for (pr, time) in vec {
                msg.push_str(&format!("\n- `{time}` {CUPRATE_GITHUB_PULL}/{pr}"));
            }

            msg.push_str("\n\n");
        }

        trace!(msg);
        RoomMessageEventContent::text_markdown(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_list(db: Arc<Database>) -> RoomMessageEventContent {
        let db = db.read().await;
        return_if_empty!(db);

        let mut string = String::new();

        // Make sure the list is sorted by going
        // through 4 passes and added in priority.
        for priority in [
            Priority::Critical,
            Priority::High,
            Priority::Medium,
            Priority::Low,
        ] {
            for (pr, md) in db.iter() {
                if md.priority == priority {
                    string += &format!("{pr:?} ");
                }
            }
        }

        trace!(string);
        RoomMessageEventContent::text_plain(string)
    }

    /// TODO
    #[instrument]
    async fn handle_json(db: Arc<Database>) -> RoomMessageEventContent {
        let db = db.read().await;
        return_if_empty!(db);

        let json = serde_json::to_string_pretty(&*db).unwrap();
        let msg = format!("```json\n{json}\n```");

        trace!(msg);
        RoomMessageEventContent::text_markdown(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_add(
        db: Arc<Database>,
        prs: Vec<PullRequest>,
        priority: Option<Priority>,
        user: OwnedUserId,
        timestamp: u64,
    ) -> RoomMessageEventContent {
        let mut db = db.write().await;

        for pr in &prs {
            // Check if PR already exists.
            if db.contains_key(pr) {
                let msg = format!("#{pr} already in queue");
                trace!(msg);
                return RoomMessageEventContent::text_plain(msg);
            }

            // Check if valid PR.
            match pr_is_open(*pr).await {
                Ok(true) => continue,
                Ok(false) => {
                    let msg = format!("#{pr} is already merged/closed");
                    trace!(msg);
                    return RoomMessageEventContent::text_plain(msg);
                }
                Err(e) => return RoomMessageEventContent::text_plain(e.to_string()),
            }
        }

        let priority = priority.unwrap_or_default();

        let metadata = PullRequestMetadata {
            priority,
            user,
            timestamp,
        };

        for pr in &prs {
            db.insert(*pr, metadata.clone());
        }

        let msg = format!("{prs:?} added to queue (priority: {priority:#?})");
        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_remove(db: Arc<Database>, prs: Vec<PullRequest>) -> RoomMessageEventContent {
        let mut db = db.write().await;

        // First, verify all PR inputs exist.
        for pr in &prs {
            if !db.contains_key(pr) {
                let err = format!("#{pr} is not in the queue, skipping !remove");
                info!(err);
                return RoomMessageEventContent::text_plain(err);
            }
        }

        // Remove all PRs.
        for pr in &prs {
            // INVARIANT: we check above this PR exists.
            db.remove(pr).unwrap();
        }

        let msg = format!("Removed {prs:?}");
        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    pub async fn handle_sweep(db: Arc<Database>) -> RoomMessageEventContent {
        let db = db.write().await;
        return_if_empty!(db);

        let mut keep = vec![];
        let mut sweep = vec![];

        for pr in db.keys() {
            // Check if open PR.
            match pr_is_open(*pr).await {
                Ok(true) => keep.push(pr),
                Ok(false) => sweep.push(pr),
                Err(e) => return RoomMessageEventContent::text_plain(e.to_string()),
            }
        }

        let msg = format!("Keeping: {keep:?}, sweeping: {sweep:?}");
        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_sweeper(db: Arc<Database>) -> RoomMessageEventContent {
        let msg = if CONFIG.sweeper == 0 {
            "Sweeper is disabled".to_string()
        } else {
            let seconds_passed = INIT_INSTANT.elapsed().as_secs() % CONFIG.sweeper;
            let secs_until_next_sweep = CONFIG.sweeper.saturating_sub(seconds_passed);
            let uptime = Uptime::from(secs_until_next_sweep);
            format!("Time until next sweep: {uptime}")
        };

        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_clear(db: Arc<Database>) -> RoomMessageEventContent {
        let mut db = db.write().await;
        return_if_empty!(db);

        let mut prs = vec![];
        while let Some((pr, _)) = db.pop_first() {
            prs.push(pr);
        }

        let msg = format!("Cleared {prs:?}");
        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    async fn handle_meeting() -> RoomMessageEventContent {
        {
            use chrono::prelude::*;
            let now = chrono::Utc::now();

            if now.date_naive().weekday() != CUPRATE_MEETING_WEEKDAY {
                let msg = format!("It is not <{CUPRATE_MEETING_WEEKDAY}>");
                trace!(msg);
                return RoomMessageEventContent::text_plain(msg);
            }

            if now.time().hour() < CUPRATE_MEETING_UTC_HOUR {
                let msg = format!("It is not >= {CUPRATE_MEETING_UTC_HOUR}:00");
                trace!(msg);
                return RoomMessageEventContent::text_plain(msg);
            }
        }

        let msg = if MEETING_ONGOING.load(Ordering::Acquire) {
            let mut logs = String::new();
            let mut db = MEETING_DATABASE.lock().await;
            std::mem::swap(&mut logs, &mut db);

            MEETING_ONGOING.store(false, Ordering::Release);

            match crate::github::finish_cuprate_meeting(logs).await {
                Ok((logs, next_meeting)) => {
                    format!("- Logs: {logs}\n - Next meeting: {next_meeting}")
                }
                Err(e) => e.to_string(),
            }
        } else {
            let mut db = MEETING_DATABASE.lock().await;
            *db = String::from("## Meeting logs");
            MEETING_ONGOING.store(true, Ordering::Release);
            TXT_MEETING_START_IDENT.to_string()
        };

        trace!(msg);
        RoomMessageEventContent::text_markdown(msg)
    }

    /// TODO
    async fn handle_agenda(items: Vec<String>) -> RoomMessageEventContent {
        let msg = match crate::github::edit_cuprate_meeting_agenda(items).await {
            Ok(url) => format!("Updated: {url}"),
            Err(e) => e.to_string(),
        };
        trace!(msg);
        RoomMessageEventContent::text_plain(msg)
    }

    /// TODO
    #[instrument]
    fn handle_status() -> RoomMessageEventContent {
        let elapsed = INIT_INSTANT.elapsed().as_secs_f32();
        let uptime = UptimeFull::from(elapsed).to_string();
        let meeting = MEETING_ONGOING.load(Ordering::Acquire);

        let msg = format!("{MOO}, meeting: {meeting}, uptime: {uptime}");

        trace!(msg);
        RoomMessageEventContent::text_markdown(msg)
    }

    /// TODO
    #[instrument]
    fn handle_help() -> RoomMessageEventContent {
        trace!(HELP);
        RoomMessageEventContent::text_markdown(HELP)
    }

    /// TODO
    #[cold]
    #[inline(never)]
    #[instrument]
    async fn handle_shutdown(db: Arc<Database>) -> RoomMessageEventContent {
        crate::shutdown::graceful_shutdown(db);
        // FIXME: use `!` when stable.
        unreachable!()
    }
}
