//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{sync::Arc, time::Duration};

use tracing::{debug, info};

use crate::database::Database;

//---------------------------------------------------------------------------------------------------- Event
/// TODO
///
/// # Panics
/// TODO
#[cold]
#[inline(never)]
pub fn spawn_sweeper(db: Arc<Database>, sweeper: u64) {
    std::thread::Builder::new()
        .name("Sweeper".to_string())
        .spawn(move || async move {
            sweeper_main(db, sweeper);
        })
        .unwrap();
}

/// TODO
#[cold]
#[inline(never)]
#[tokio::main]
#[allow(clippy::missing_panics_doc, clippy::needless_pass_by_value)]
async fn sweeper_main(db: Arc<Database>, sweeper: u64) {
    let duration = Duration::from_secs(sweeper);

    loop {
        if db.read().await.is_empty() {
            info!("Sweeper: skipping, empty DB");
        } else {
            let msg = crate::command::Command::handle_sweep(Arc::clone(&db)).await;
            info!("Sweeper: {msg:?}");
            crate::free::send(msg).await.unwrap();
        }

        debug!("Sweeper: sleeping for {duration:?}");
        tokio::time::sleep(duration).await;
    }
}
