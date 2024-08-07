//! TODO

//---------------------------------------------------------------------------------------------------- Use
use anyhow::anyhow;
use matrix_sdk::ruma::OwnedUserId;
use serde::{Deserialize, Serialize};

use crate::constants::{
    ALLOWED_MATRIX_IDS_DEFAULT, CONFIG_PATH, MOO_CONFIG_PATH, MOO_GITHUB_TOKEN_ENV_VAR,
    MOO_MATRIX_ID, MOO_PASSWORD_ENV_VAR,
};

//----------------------------------------------------------------------------------------------------
/// TODO
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Config {
    /// TODO
    #[serde(default = "default_password")]
    pub password: String,

    /// TODO
    #[serde(default = "default_password")]
    pub token: String,

    /// TODO
    #[serde(default = "default_allowed_users")]
    pub allowed_users: Vec<OwnedUserId>,

    /// TODO
    #[serde(default = "default_sweeper")]
    pub sweeper: u64,

    /// TODO
    #[serde(default = "default_sweep_on_startup")]
    pub sweep_on_startup: bool,

    /// TODO
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            password: default_password(),
            token: default_password(),
            allowed_users: default_allowed_users(),
            sweeper: default_sweeper(),
            sweep_on_startup: default_sweep_on_startup(),
            log_level: default_log_level(),
        }
    }
}

impl Config {
    /// TODO
    ///
    /// # Errors
    /// TODO
    ///
    ///
    #[cold]
    #[inline(never)]
    pub fn open() -> Result<Self, anyhow::Error> {
        println!("Reading config from: {:?}", &*MOO_CONFIG_PATH);

        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(&*CONFIG_PATH)?;

        // Read config or get default.
        let mut this = if let Ok(vec) = std::fs::read(&*MOO_CONFIG_PATH) {
            let string = String::from_utf8(vec)?;
            toml::from_str(&string)?
        } else {
            println!("No config found, using default");
            Self::default()
        };

        if let Ok(password) = std::env::var(MOO_PASSWORD_ENV_VAR) {
            println!("Using environment variable: `{MOO_PASSWORD_ENV_VAR}`");
            this.password = password;
        }

        if this.password.is_empty() {
            return Err(anyhow!("`{}` password was empty", &*MOO_MATRIX_ID));
        }

        if let Ok(token) = std::env::var(MOO_GITHUB_TOKEN_ENV_VAR) {
            println!("Using environment variable: `{MOO_GITHUB_TOKEN_ENV_VAR}`");
            this.token = token;
        }

        if this.token.is_empty() {
            eprintln!("GitHub token was empty, API access will not work...!");
        }

        Ok(this)
    }
}

//---------------------------------------------------------------------------------------------------- Free
/// TODO
const fn default_password() -> String {
    String::new()
}

/// TODO
fn default_allowed_users() -> Vec<OwnedUserId> {
    ALLOWED_MATRIX_IDS_DEFAULT.to_vec()
}

/// TODO
const fn default_sweeper() -> u64 {
    // 1 day.
    24 * 60 * 60
}

/// TODO
const fn default_sweep_on_startup() -> bool {
    true
}

/// TODO
fn default_log_level() -> String {
    String::from("TRACE")
}
