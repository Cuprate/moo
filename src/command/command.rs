//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::fmt::{Display, Write};

use strum::{AsRefStr, EnumCount, EnumIs, EnumIter, FromRepr, IntoStaticStr, VariantNames};

use crate::{priority::Priority, pull_request::PullRequest};

//---------------------------------------------------------------------------------------------------- Command
/// TODO
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    AsRefStr,
    EnumCount,
    EnumIs,
    EnumIter,
    FromRepr,
    IntoStaticStr,
    VariantNames,
)]
#[repr(u8)]
#[strum(prefix = "!")]
#[strum(serialize_all = "lowercase")]
pub enum Command {
    /// `!queue`
    Queue,
    /// `!list`
    List,
    /// `!json`
    Json,
    /// `!add <PR_NUMBERS> [PRIORITY]`
    Add((Vec<PullRequest>, Option<Priority>)),
    /// `!remove <PR_NUMBERS>`
    Remove(Vec<PullRequest>),
    /// `!sweep`
    Sweep,
    /// `!sweeper`
    Sweeper,
    /// `!clear`
    Clear,
    /// `!status`
    Status,
    /// `!help`
    Help,
    /// `!shutdown`
    Shutdown,
}

impl Command {
    /// TODO
    pub const PREFIX: char = '!';
}

//---------------------------------------------------------------------------------------------------- Trait
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(Self::PREFIX)?;

        match self {
            Self::Queue => f.write_str("queue"),
            Self::List => f.write_str("list"),
            Self::Json => f.write_str("json"),
            Self::Sweep => f.write_str("sweep"),
            Self::Sweeper => f.write_str("sweeper"),
            Self::Clear => f.write_str("clear"),
            Self::Status => f.write_str("status"),
            Self::Help => f.write_str("help"),
            Self::Shutdown => f.write_str("shutdown"),

            // Add
            Self::Add((prs, priority)) => {
                f.write_str("add")?;
                for pr in prs {
                    f.write_fmt(format_args!(" {pr}"))?;
                }
                if let Some(p) = priority {
                    f.write_fmt(format_args!(" {p}"))?;
                }
                Ok(())
            }

            // Remove
            Self::Remove(prs) => {
                f.write_str("remove")?;
                for pr in prs {
                    f.write_fmt(format_args!(" {pr}"))?;
                }
                Ok(())
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::priority::Priority;

    /// Test `Command`'s `Display` impl.
    #[test]
    fn display() {
        for (command, expected) in [
            (Command::Queue, "!queue"),
            (Command::List, "!list"),
            (Command::Json, "!json"),
            (
                Command::Add((vec![1, 2], Some(Priority::Low))),
                "!add 1 2 low",
            ),
            (Command::Add((vec![1, 2], None)), "!add 1 2"),
            (Command::Remove(vec![45, 2]), "!remove 45 2"),
            (Command::Sweep, "!sweep"),
            (Command::Sweeper, "!sweeper"),
            (Command::Clear, "!clear"),
            (Command::Status, "!status"),
            (Command::Help, "!help"),
            (Command::Shutdown, "!shutdown"),
        ] {
            assert_eq!(command.to_string(), expected);
        }
    }
}
