//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{collections::BTreeSet, fmt::Debug};

use matrix_sdk::ruma::events::MessageLikeEventContent;
use tracing::{info, trace};

use crate::constants::{MOO_DB_BACKUP_PATH, MOO_DB_PATH, ROOM};

//---------------------------------------------------------------------------------------------------- Main
/// [`Box`] leak a `T`.
pub fn leak<T>(t: T) -> &'static T {
    Box::leak(Box::new(t))
}

/// Returns `true` if a slice contains duplicates.
pub fn slice_contains_duplicates<T: Ord>(slice: &[T]) -> bool {
    let btree = slice.iter().collect::<BTreeSet<&T>>();
    btree.len() != slice.len()
}

/// TODO
///
/// # Errors
/// TODO
pub fn backup_db() -> Result<(), anyhow::Error> {
    info!("backing up DB");

    let db = std::fs::read(&*MOO_DB_PATH)?;
    std::fs::write(&*MOO_DB_BACKUP_PATH, db)?;

    Ok(())
}

/// TODO
///
/// # Errors
/// TODO
#[inline]
pub async fn send(
    msg: impl MessageLikeEventContent + Debug + Send + Sync + 'static,
) -> Result<(), anyhow::Error> {
    trace!("sending msg: {msg:#?}");
    ROOM.send(msg).await?;
    Ok(())
}
