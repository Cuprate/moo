//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use matrix_sdk::ruma::{
    events::{
        room::message::{MessageType, RoomMessageEventContent, SyncRoomMessageEvent},
        MessageLikeEventType, SyncMessageLikeEvent,
    },
    OwnedUserId,
};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::{
    command::Command,
    constants::{CONFIG, MOO_MATRIX_ID, ROOM},
    database::Database,
    free::send,
};

//---------------------------------------------------------------------------------------------------- Event
/// TODO
#[instrument]
#[inline]
pub async fn room_message_handler(
    startup: SystemTime,
    db: Arc<Database>,
    event: SyncRoomMessageEvent,
) {
    trace!("room_message_handler()");

    let Some((sender, message, timestamp)) = filter_message(startup, event) else {
        return;
    };

    let Some(command) = parse_command(message).await else {
        return;
    };

    handle_command(db, command, sender, timestamp).await;
}

/// TODO
#[inline]
fn filter_message(
    startup: SystemTime,
    event: SyncRoomMessageEvent,
) -> Option<(OwnedUserId, String, u64)> {
    if event.event_type() != MessageLikeEventType::RoomMessage {
        trace!("Ignoring non-message event");
        return None;
    }

    let SyncMessageLikeEvent::Original(event) = event else {
        info!("Redacted event, skipping: {event:#?}");
        return None;
    };

    if event.content.body().contains("moo") && moo() {
        info!("Ignoring message, moo is more important");
        return None;
    }

    let sender = event.sender;
    let origin_server_ts = event.origin_server_ts;

    // The fact that this is after `moo()` means that there's a
    // chance `moo` `moo()`ing will trigger itself to `moo()` again...
    // and again... and again... and again...
    if sender == *MOO_MATRIX_ID {
        info!("Ignoring self message");
        return None;
    }

    // This is checked elsewhere but checking this here
    // will filter 99% of messages at a much earlier stage.
    if !event.content.body().starts_with(Command::PREFIX) {
        info!("Ignoring non-command message");
        return None;
    }

    if !CONFIG.allowed_users.contains(&sender) {
        info!("Ignoring message from non-allowed user: {sender}");
        return None;
    }

    let Some(origin_server_ts) = origin_server_ts.to_system_time() else {
        warn!("Event UNIX time could not be parsed: {origin_server_ts:#?}");
        return None;
    };

    if let Ok(duration) = startup.duration_since(origin_server_ts) {
        let s = duration.as_secs_f32();
        info!("Ignoring previous session message: {s}s ago");
        return None;
    }

    let Ok(timestamp) = origin_server_ts.duration_since(UNIX_EPOCH) else {
        warn!("Timestamp could not be parsed: {origin_server_ts:#?}");
        return None;
    };

    let MessageType::Text(text) = &event.content.msgtype else {
        trace!("Ignoring non-text event");
        return None;
    };

    Some((sender, text.body.clone(), timestamp.as_secs()))
}

/// TODO
#[inline]
async fn parse_command(msg: String) -> Option<Command> {
    trace!("Attempting to parse command: {msg}");

    match Command::from_str(&msg) {
        Ok(cmd) => Some(cmd),
        Err(e) => {
            let msg = format!("Command parse error: {e:?}");
            error!("{msg}");

            if let Err(e) = ROOM.send(RoomMessageEventContent::text_plain(msg)).await {
                error!("Could not send error response to Matrix: {e}");
            }

            None
        }
    }
}

/// TODO
#[inline]
async fn handle_command(db: Arc<Database>, command: Command, sender: OwnedUserId, timestamp: u64) {
    match command.handle(&db, sender, timestamp).await {
        Ok(()) => {
            db.backup_and_save().await;
            debug!("Command action success");
        }
        Err(e) => error!("Command action fail: {e:?}"),
    }
}

/// For you, moneromooo.
#[cold]
#[inline(never)]
fn moo() -> bool {
    use rand::prelude::*;

    // 0.390625% chance
    let random_number = rand::thread_rng().gen::<u8>();
    trace!("moo(): random_number: {random_number}");
    let time_to_moo = random_number == 0;

    if time_to_moo {
        let o = rand::thread_rng().gen::<u8>();
        let mut moo = String::with_capacity(usize::from(2 + o));

        moo += "moo";
        for _ in 0..o {
            moo += "o";
        }

        trace!(moo);
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                if let Err(e) = send(RoomMessageEventContent::text_plain(moo)).await {
                    error!("critical error: we could not moo: {e}");
                }
            });
        });
    }

    time_to_moo
}
