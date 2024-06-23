//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::sync::Arc;

use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use readable::up::UptimeFull;
use tracing::{instrument, trace, warn};

use crate::{constants::INIT_INSTANT, database::Database, free::send};

//---------------------------------------------------------------------------------------------------- Event
/// TODO
///
/// # Panics
/// TODO
#[cold]
#[inline(never)]
#[instrument]
pub fn graceful_shutdown(db: Arc<Database>) {
    trace!("shutdown()");

    let elapsed = INIT_INSTANT.elapsed().as_secs_f32();
    let uptime = UptimeFull::from(elapsed);
    let msg = format!("Shutting down gracefully, uptime: {uptime}");

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            send(RoomMessageEventContent::text_plain(msg))
                .await
                .unwrap();
        });
    });

    // FIXME: use `-> !` when stable.
    std::process::exit(0);
}
