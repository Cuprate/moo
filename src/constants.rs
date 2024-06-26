//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{hint::black_box, path::PathBuf, time::Instant};

use const_format::formatcp;
use matrix_sdk::{
    ruma::{user_id, OwnedRoomId, OwnedUserId, RoomId},
    Client, Room,
};
use once_cell::sync::Lazy;
use tracing::info;

use crate::config::Config;

//---------------------------------------------------------------------------------------------------- URL
/// TODO
pub const CUPRATE_GITHUB_PULL: &str = "https://github.com/Cuprate/cuprate/pull";

/// TODO
pub const CUPRATE_GITHUB_PULL_API: &str = "https://api.github.com/repos/Cuprate/cuprate/pulls";

/// TODO
pub const MONERO_META_GITHUB_ISSUE_API: &str =
    "https://api.github.com/repos/monero-project/meta/issues";

/// TODO
pub const MONERO_META_GITHUB_ISSUE: &str = "https://github.com/monero-project/meta/issues";

// /// TODO
// pub const MONERO_META_GITHUB_ISSUE_API: &str =
//     "https://api.github.com/repos/hinto-janai/labeler-test/issues";

// /// TODO
// pub const MONERO_META_GITHUB_ISSUE: &str = "https://github.com/hinto-janai/labeler-test/issues";

//---------------------------------------------------------------------------------------------------- Version
/// Build commit.
///
/// This needs to be set with the environment variable `COMMIT`.
/// It used to be just an `include_str!()` to the `main` branch but
/// CI running on PR branches with different branch names messes it up.
///
/// This should get set automatically in `build.rs`.
pub const COMMIT: &str = env!("COMMIT");

/// Build profile (debug/release).
pub const BUILD: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};

/// `moo` version.
pub const MOO_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `moo` name + version.
pub const MOO_NAME_VER: &str = concat!("moo v", env!("CARGO_PKG_VERSION"));

/// TODO
pub const MOO: &str = formatcp!("{MOO_NAME_VER}, {COMMIT} ({BUILD})");

/// TODO
pub const MOO_USER_AGENT: &str = concat!("moo", "/", env!("CARGO_PKG_VERSION"),);

//---------------------------------------------------------------------------------------------------- Matrix Rooms
/// Cuprate's Matrix room ID.
pub static CUPRATE_MATRIX_ROOM_ID: Lazy<OwnedRoomId> =
    Lazy::new(|| RoomId::parse("!zPLCnZSsyeFFxUiqUZ:monero.social").unwrap());

// /// Test Matrix room ID.
// pub static CUPRATE_MATRIX_ROOM_ID: Lazy<OwnedRoomId> =
//     Lazy::new(|| RoomId::parse("!SrjNVhHuHOWcFfYRfj:monero.social").unwrap());

//---------------------------------------------------------------------------------------------------- IDs
/// TODO
pub const MOO_GITHUB_ID: &str = "moo900";

/// TODO
pub static MOO_MATRIX_ID: Lazy<OwnedUserId> =
    Lazy::new(|| user_id!("@moo:monero.social").to_owned());

/// TODO
pub static ALLOWED_MATRIX_IDS_DEFAULT: Lazy<Vec<OwnedUserId>> = Lazy::new(|| {
    vec![
        user_id!("@hinto:monero.social").to_owned(),
        user_id!("@boog900:monero.social").to_owned(),
        user_id!("@syntheticbird:monero.social").to_owned(),
        user_id!("@yamabiiko:unitoo.it").to_owned(),
    ]
});

//---------------------------------------------------------------------------------------------------- Misc
/// TODO
pub const MOO_PASSWORD_ENV_VAR: &str = "MOO_PASSWORD";

/// TODO
pub const MOO_GITHUB_TOKEN_ENV_VAR: &str = "MOO_GITHUB_TOKEN";

/// TODO
pub const DEFAULT_LOG_LEVEL: tracing::Level = tracing::Level::TRACE;

/// TODO
pub const CUPRATE_MEETING_WEEKDAY: chrono::Weekday = chrono::Weekday::Tue;

/// TODO
pub const CUPRATE_MEETING_UTC_HOUR: u32 = 18;

//---------------------------------------------------------------------------------------------------- PATHs
/// TODO
pub const MOO_SUBDIR: &str = "moo";

/// TODO
pub const DB_FILENAME: &str = "moo.json";

/// TODO
pub const DB_BACKUP_FILENAME: &str = "moo.backup.json";

/// TODO
pub const CONFIG_FILENAME: &str = "moo.toml";

/// TODO
pub static DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::data_dir().unwrap();
    path.push(MOO_SUBDIR);
    path
});

/// TODO
pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::config_dir().unwrap();
    path.push(MOO_SUBDIR);
    path
});

/// TODO
pub static MOO_DB_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DATA_PATH.clone();
    path.push(DB_FILENAME);
    path
});

/// TODO
pub static MOO_DB_BACKUP_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DATA_PATH.clone();
    path.push(DB_BACKUP_FILENAME);
    path
});

/// TODO
pub static MOO_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CONFIG_PATH.clone();
    path.push(CONFIG_FILENAME);
    path
});

//---------------------------------------------------------------------------------------------------- Text
/// TODO
pub const TXT_EMPTY: &str = "<empty>";

/// TODO
pub const HELP: &str = r"| Command                        | Description |
|--------------------------------|-------------|
| `!queue`                       | Report the queue as a markdown list. Sorted by priority, then add time.
| `!list`                        | Report the queue as a simple list from high to low priority.
| `!json`                        | Report the queue as JSON.
| `!add <PR_NUMBERS> [PRIORITY]` | Add PR(s) to the queue. `PRIORITY` is `low/medium/high/critical` (default = medium).
| `!remove <PR_NUMBERS>`         | Remove PR(s) from the queue.
| `!sweep`                       | Remove any PRs in the queue that can be removed (since they were merged).
| `!sweeper`                     | Report how long before an automatic `!sweep` occurs.
| `!clear`                       | Clear the entire queue.
| `!meeting`                     | Begin/end Cuprate meetings. Issues/logs will be handled automatically after ending.
| `!agenda <ARRAY_OF_STRINGS>`   | Re-write the current Cuprate meeting's extra agenda items.
| `!status`                      | Report `moo` status.
| `!help`                        | Print all `moo` commands.
| `!shutdown`                    | Shutdown `moo`.";

/// TODO
pub const TXT_CUPRATE_MEETING_PREFIX: &str = "[Cuprate](https://github.com/Cuprate/cuprate) is an effort to create an alternative Monero node implementation.

Location: [Libera.chat, #cuprate](https://libera.chat/) | [Matrix](https://matrix.to/#/#cuprate:monero.social?via=matrix.org&via=monero.social)

> Note that there are currently communication issues with Matrix accounts created on the matrix.org server, consider using a different homeserver to see messages.

[Join the Monero Matrix server if you don't already have a Matrix account](https://www.getmonero.org/resources/user-guides/join-monero-matrix.html)

Time: 18:00 UTC [Check in your timezone](https://www.timeanddate.com/worldclock/converter.html)

Moderator: @Boog900

Please comment on GitHub in advance of the meeting if you would like to propose a discussion topic.

Main discussion topics:

- Greetings
- Updates: What is everyone working on?
- Project: What is next for Cuprate?";

/// TODO
pub const TXT_CUPRATE_MEETING_SUFFIX: &str = "- Any other business";

/// TODO
pub const TXT_MEETING_START_IDENT: &str = "Recording meeting logs...";

//---------------------------------------------------------------------------------------------------- Statics
// These are accessed everywhere and replace function inputs.

/// Startup instant.
pub static INIT_INSTANT: Lazy<Instant> = Lazy::new(Instant::now);

/// TODO
pub static INIT: Lazy<(Config, Client, Room)> = Lazy::new(|| {
    let mut config = Config::open().unwrap();

    crate::logger::init_logger(&config.log_level);

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async move {
            info!(
                "Creating matrix client for: {}",
                MOO_MATRIX_ID.server_name()
            );

            // Create client.
            let client = Client::builder()
                .server_name(MOO_MATRIX_ID.server_name())
                .build()
                .await
                .unwrap();

            info!("Logging into: {}", MOO_MATRIX_ID.as_str());

            // Log in.
            client
                .matrix_auth()
                .login_username(MOO_MATRIX_ID.as_str(), &config.password)
                .send()
                .await
                .unwrap();

            info!("Joining room: {}", CUPRATE_MATRIX_ROOM_ID.as_str());

            // Join the Cuprate room.
            let room = client
                .join_room_by_id(&CUPRATE_MATRIX_ROOM_ID)
                .await
                .unwrap();

            // Zeroize config password.
            {
                let mut dummy = black_box(String::new());
                std::mem::swap(black_box(&mut dummy), black_box(&mut config.password));
                let _zero = black_box(zeroize::Zeroizing::new(dummy));
            }
            assert!(config.password.is_empty());

            (config, client, room)
        })
    })
});

/// TODO
pub static CONFIG: Lazy<&'static Config> = Lazy::new(|| &INIT.0);

/// TODO
pub static CLIENT: Lazy<&'static Client> = Lazy::new(|| &INIT.1);

/// TODO
pub static ROOM: Lazy<&'static Room> = Lazy::new(|| &INIT.2);
