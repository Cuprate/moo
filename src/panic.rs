//! TODO

//---------------------------------------------------------------------------------------------------- Use

use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use readable::up::UptimeFull;

use crate::constants::INIT_INSTANT;

//----------------------------------------------------------------------------------------------------
/// Set the custom panic hook.
#[cold]
#[inline(never)]
#[allow(clippy::missing_panics_doc)]
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |panic_info| {
        // Set stack-trace.
        let stack_trace = std::backtrace::Backtrace::force_capture();
        let uptime = UptimeFull::from(&*INIT_INSTANT);

        // Re-format panic info.
        let panic_info = format!(
            "```
Panic, shutting down:

Uptime:\n{uptime}

Panic info: {panic_info:#?}

Stack backtrace:\n{stack_trace}
```",
        );

        println!("{panic_info}");

        // Send panic message to room.
        tokio::runtime::Handle::current().block_on(async move {
            let msg = RoomMessageEventContent::text_markdown(panic_info);
            crate::free::send(msg).await.unwrap();
        });
    }));
}
