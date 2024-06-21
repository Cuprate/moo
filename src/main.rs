use std::{sync::Arc, time::SystemTime};

use matrix_sdk::{
    config::SyncSettings,
    ruma::{
        events::{
            room::message::{MessageType, RoomMessageEventContent, SyncRoomMessageEvent},
            MessageLikeEventType,
        },
        user_id, RoomId,
    },
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let startup = SystemTime::now();

    let moo = user_id!("@moo:monero.social");
    let client = Client::builder()
        .server_name(moo.server_name())
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(moo, "pass")
        .send()
        .await?;

    let room_id = RoomId::parse("!zPLCnZSsyeFFxUiqUZ:monero.social").unwrap();
    let room = Arc::new(client.join_room_by_id(&room_id).await?);

    room.clone()
        .add_event_handler(move |ev: SyncRoomMessageEvent| async move {
            if ev.event_type() != MessageLikeEventType::RoomMessage {
                return;
            }

            let Some(origin_server_ts) = ev.origin_server_ts().to_system_time() else {
                return;
            };

            if startup > origin_server_ts {
                println!("ignoring: {startup:?} > {origin_server_ts:?}");
                return;
            }

            if ev.sender() != "@hinto:monero.social" {
                println!("1");
                return;
            }

            let Some(ev) = ev.as_original() else {
                println!("2");
                return;
            };

            let MessageType::Text(text) = &ev.content.msgtype else {
                println!("3");
                return;
            };

            let text = text.body.as_str();
            println!("{text}");

            if text == "!queue" {
                let db = std::fs::read("/tmp/db").unwrap();
                let s = String::from_utf8(db).unwrap();
                if s.is_empty() || s == " " {
                    let msg = RoomMessageEventContent::text_plain("nothing");
                    room.send(msg).await.unwrap();
                } else {
                    let msg = RoomMessageEventContent::text_plain(s.to_string());
                    room.send(msg).await.unwrap();
                }
            } else if text.starts_with("!add") {
                let mut text = text.split_whitespace();
                text.next();
                let number: u64 = text.next().unwrap().parse().unwrap();
                let db = std::fs::read("/tmp/db").unwrap();
                let s = String::from_utf8(db).unwrap();
                let s = format!("{s}{number} ");
                println!("{s}");
                std::fs::write("/tmp/db", s).unwrap();
            } else {
                println!("nope");
            }
        });

    client.sync(SyncSettings::default()).await?;

    Ok(())
}
