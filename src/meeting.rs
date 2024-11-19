//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{sync::atomic::AtomicBool, time::SystemTime};

use matrix_sdk::ruma::events::{
    room::message::{MessageType, SyncRoomMessageEvent},
    MessageLikeEventType, SyncMessageLikeEvent,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tracing::{info, instrument, trace, warn};

use crate::{
    command::Command,
    constants::{CONFIG, TXT_MEETING_START_IDENT},
};

//---------------------------------------------------------------------------------------------------- Event
/// TODO
pub static MEETING_ONGOING: AtomicBool = AtomicBool::new(false);

/// TODO
pub static MEETING_DATABASE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

//---------------------------------------------------------------------------------------------------- Event
/// TODO
#[instrument]
#[inline]
pub async fn meeting_handler(startup: SystemTime, event: SyncRoomMessageEvent) {
    trace!("meeting_handler()");

    if !MEETING_ONGOING.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }

    if event.event_type() != MessageLikeEventType::RoomMessage {
        trace!("Ignoring non-message event");
    }

    let SyncMessageLikeEvent::Original(event) = event else {
        info!("Redacted event, skipping: {event:#?}");
        return;
    };

    let sender = event.sender;
    let origin_server_ts = event.origin_server_ts;

    let Some(origin_server_ts) = origin_server_ts.to_system_time() else {
        warn!("Event UNIX time could not be parsed: {origin_server_ts:#?}");
        return;
    };

    if let Ok(duration) = startup.duration_since(origin_server_ts) {
        let s = duration.as_secs_f32();
        info!("Ignoring previous session message: {s}s ago");
        return;
    }

    {
        let body = event.content.body();
        if CONFIG.allowed_users.contains(&sender) && body == Command::Meeting.as_ref()
            || body == TXT_MEETING_START_IDENT
        {
            info!("Ignoring meeting ident");
            return;
        }
    }

    let text = match &event.content.msgtype {
        MessageType::Text(t) => &t.body,
        MessageType::Audio(_) => "<audio>",
        MessageType::Emote(_) => "<emote>",
        MessageType::Image(_) => "<image>",
        MessageType::Video(_) => "<video>",
        MessageType::File(_) => "<file>",
        _ => "<unknown_attachment>",
    };

    // HACK: do not relay ``` as it messes up the meeting log formatting:
    // <https://github.com/monero-project/meta/issues/1108>
    if text == "```" {
        info!("Ignoring ```");
        return;
    }

    let mut db = MEETING_DATABASE.lock().await;
    if MEETING_ONGOING.load(std::sync::atomic::Ordering::Acquire) {
        *db += &format!("\n```\n{}: {text}\n```", sender.localpart());
    }
}
