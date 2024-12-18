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
    constants::{CONFIG, MOO_MATRIX_ID, TXT_MEETING_START_IDENT},
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

        let is_meeting_starting_cmd =
            || CONFIG.allowed_users.contains(&sender) && body == Command::Meeting.as_ref();

        let is_meeting_starting_msg =
            || sender == *MOO_MATRIX_ID && body.starts_with(TXT_MEETING_START_IDENT);

        if is_meeting_starting_cmd() || is_meeting_starting_msg() {
            info!("Ignoring meeting starting cmd/msg");
            return;
        }
    }

    let text = match event.content.msgtype {
        MessageType::Text(t) => t.body,
        MessageType::Emote(x) => x.body,
        MessageType::Notice(x) => x.body,
        MessageType::ServerNotice(x) => x.body,
        MessageType::VerificationRequest(x) => x.body,
        MessageType::Location(x) => format!("<{}>", x.plain_text_representation()),
        MessageType::Audio(x) => format!("<{}>", x.filename()),
        MessageType::File(x) => format!("<{}>", x.filename()),
        MessageType::Image(x) => format!("<{}>", x.filename()),
        MessageType::Video(x) => format!("<{}>", x.filename()),
        _ => "<unknown>".to_string(),
    };

    // HACK: do not relay ``` as it messes up the meeting log formatting:
    // <https://github.com/monero-project/meta/issues/1108>
    let text = if text.contains("```") {
        text.replace("```", "")
    } else {
        text
    };

    let mut db = MEETING_DATABASE.lock().await;
    if MEETING_ONGOING.load(std::sync::atomic::Ordering::Acquire) {
        db.push_str(&format!("\n```\n{}: {text}\n```", sender.localpart()));
    }
}
