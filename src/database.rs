//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    sync::Arc,
};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::{debug, error, info, instrument};

use crate::{
    constants::{DATA_PATH, MOO_DB_PATH},
    pull_request::{PullRequest, PullRequestMetadata},
};

//---------------------------------------------------------------------------------------------------- Free functions
/// TODO
pub type DatabaseInner = BTreeMap<PullRequest, PullRequestMetadata>;

//---------------------------------------------------------------------------------------------------- Free functions
/// TODO
#[derive(Debug)]
pub struct Database {
    /// TODO
    pub inner: RwLock<DatabaseInner>,
}

impl Database {
    /// TODO
    ///
    /// Returns `Arc<Self>` instead of `&'static Self`
    /// since [`Drop`] runs [`Self::save`] and that
    /// does not get called with `static`s.
    ///
    /// # Errors
    /// TODO
    #[cold]
    #[inline(never)]
    pub fn open() -> Result<Arc<Self>, anyhow::Error> {
        info!("Opening DB at: {:?}", &*MOO_DB_PATH);

        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(&*DATA_PATH)?;

        // Open the database file, create if needed.
        let mut db_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&*MOO_DB_PATH)?;

        // Assert we can deserialize the DB.
        let mut vec = vec![];
        db_file.read_to_end(&mut vec)?;
        let string = String::from_utf8(vec)?;
        let inner: DatabaseInner = if string.is_empty() {
            DatabaseInner::new()
        } else {
            serde_json::from_str(&string)?
        };
        debug!("Opened DB: {string:#?}");

        let this = Self {
            inner: RwLock::new(inner),
        };

        Ok(Arc::new(this))
    }

    /// TODO
    ///
    /// # Errors
    /// TODO
    #[allow(clippy::missing_panics_doc)] // rwlock
    pub async fn save(&self) -> Result<String, anyhow::Error> {
        // Open the database file, create if needed.
        let mut db_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*MOO_DB_PATH)
            .unwrap();

        // Serialize the DB to disk.
        let inner = self.inner.read().await;
        let string = serde_json::to_string_pretty(&*inner)?;
        db_file.write_all(string.as_bytes())?;

        Ok(string)
    }

    /// TODO
    #[instrument]
    pub async fn backup_and_save(&self) {
        // Backup current on-disk copy of DB.
        if let Err(e) = crate::free::backup_db() {
            error!("DB backup failed: {e}");
        }

        // Save to disk.
        match self.save().await {
            Ok(string) => debug!("Saved DB to disk: {string}"),
            Err(e) => panic!("{e}"),
        }
    }

    /// TODO
    pub async fn read(&self) -> RwLockReadGuard<'_, DatabaseInner> {
        self.inner.read().await
    }

    /// TODO
    pub async fn write(&self) -> RwLockWriteGuard<'_, DatabaseInner> {
        self.inner.write().await
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                match self.save().await {
                    Ok(string) => debug!("Saved DB to disk on shutdown: {string}"),
                    Err(e) => error!("Failed to save DB to disk on shutdown: {e}"),
                }
            });
        });
    }
}
