//! TODO

//---------------------------------------------------------------------------------------------------- Use
use std::{num::NonZero, str::FromStr};

use serde_json::Value;
use strum::VariantNames;
use tracing::debug;

use crate::{
    command::{Command, CommandParseError},
    free::slice_contains_duplicates,
    priority::Priority,
};

//---------------------------------------------------------------------------------------------------- Command
impl Command {
    /// TODO
    #[inline]
    fn from_str_add<'a, I: Iterator<Item = &'a str>>(
        mut iter: I,
    ) -> Result<Self, CommandParseError> {
        // Check for at least parameter.
        let Some(pr_number) = iter.next() else {
            return Err(CommandParseError::MissingParameter);
        };

        // Init variables.
        let mut pr_numbers = vec![];
        let mut priority = None;

        // Parse PR number.
        match pr_number.parse::<u64>() {
            Ok(pr) => pr_numbers.push(pr),
            Err(_) => return Err(CommandParseError::IncorrectParameter),
        }

        // Parse rest of parameters.
        #[allow(clippy::while_let_on_iterator)] // need access to `iter` after
        while let Some(next) = iter.next() {
            // Parse PR number and continue.
            if let Ok(pr) = next.parse::<u64>() {
                pr_numbers.push(pr);
                continue;
            }

            // Else, assume we're done. Check if priority exists.
            match serde_plain::from_str::<Priority>(next) {
                Ok(p) => {
                    priority = Some(p);
                    break;
                }
                Err(_) => return Err(CommandParseError::IncorrectParameter),
            };
        }

        // There should be no parameters after this.
        if iter.next().is_some() {
            return Err(CommandParseError::UnknownParameter);
        }

        // Error on duplicate parameters.
        if slice_contains_duplicates(&pr_numbers) {
            return Err(CommandParseError::DuplicateParameter);
        }

        Ok(Self::Add((pr_numbers, priority)))
    }

    /// TODO
    #[inline]
    fn from_str_remove<'a, I: Iterator<Item = &'a str>>(
        iter: I,
    ) -> Result<Self, CommandParseError> {
        let mut vec = vec![];

        for pr_number in iter {
            let Ok(pr_number) = pr_number.parse::<u64>() else {
                return Err(CommandParseError::IncorrectParameter);
            };

            vec.push(pr_number);
        }

        if vec.is_empty() {
            return Err(CommandParseError::MissingParameter);
        }

        // Error on duplicate parameters.
        if slice_contains_duplicates(&vec) {
            return Err(CommandParseError::DuplicateParameter);
        }

        Ok(Self::Remove(vec))
    }

    /// TODO
    #[inline]
    fn from_str_cancel<'a, I: Iterator<Item = &'a str>>(
        mut iter: I,
    ) -> Result<Self, CommandParseError> {
        let Some(count) = iter.next() else {
            return Err(CommandParseError::MissingParameter);
        };

        // Parse count number.
        let count = match count.parse::<u8>() {
            Ok(c) => NonZero::new(c).unwrap_or(NonZero::<u8>::MIN),
            Err(_) => return Err(CommandParseError::IncorrectParameter),
        };

        let mut json = String::new();
        for item in iter {
            json += " ";
            json += item;
        }

        // Parse reason.
        let reason = if json.is_empty() {
            None
        } else {
            let Ok(reason) = serde_json::from_str::<Value>(&json) else {
                return Err(CommandParseError::IncorrectParameter);
            };

            let Value::String(reason) = reason else {
                return Err(CommandParseError::IncorrectParameter);
            };

            Some(reason)
        };

        Ok(Self::Cancel(count, reason))
    }

    /// TODO
    #[inline]
    fn from_str_agenda<'a, I: Iterator<Item = &'a str>>(
        iter: I,
    ) -> Result<Self, CommandParseError> {
        let mut json = String::new();
        for item in iter {
            json += " ";
            json += item;
        }

        if json.is_empty() {
            return Err(CommandParseError::MissingParameter);
        }

        let Ok(items) = serde_json::from_str::<Vec<Value>>(&json) else {
            return Err(CommandParseError::IncorrectParameter);
        };

        let mut vec = vec![];

        for item in items {
            let Value::String(item) = item else {
                return Err(CommandParseError::IncorrectParameter);
            };

            vec.push(item);
        }

        if vec.is_empty() {
            return Err(CommandParseError::MissingParameter);
        }

        // Error on duplicate parameters.
        if slice_contains_duplicates(&vec) {
            return Err(CommandParseError::DuplicateParameter);
        }

        Ok(Self::Agenda(vec))
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let Some(command) = iter.next() else {
            debug!("Command::from_str(): missing command");
            return Err(CommandParseError::MissingCommand);
        };

        if !Self::VARIANTS.contains(&command) {
            debug!("Command::from_str(): unknown command variant");
            return Err(CommandParseError::UnknownCommand);
        }

        let this = match command {
            "!queue" => Self::Queue,
            "!list" => Self::List,
            "!json" => Self::Json,
            "!add" => Self::from_str_add(iter)?,
            "!remove" => Self::from_str_remove(iter)?,
            "!sweep" => Self::Sweep,
            "!sweeper" => Self::Sweeper,
            "!clear" => Self::Clear,
            "!meeting" => Self::Meeting,
            "!cancel" => Self::from_str_cancel(iter)?,
            "!agenda" => Self::from_str_agenda(iter)?,
            "!status" => Self::Status,
            "!help" => Self::Help,
            "!shutdown" => Self::Shutdown,
            _ => return Err(CommandParseError::UnknownCommand),
        };

        Ok(this)
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::priority::Priority;

    /// Test `FromStr` for `Command`s with no parameters.
    #[test]
    fn parse_no_params() {
        assert_eq!(Command::from_str("!queue").unwrap(), Command::Queue);
        assert_eq!(Command::from_str("!list").unwrap(), Command::List);
        assert_eq!(Command::from_str("!json").unwrap(), Command::Json);
        assert_eq!(Command::from_str("!sweep").unwrap(), Command::Sweep);
        assert_eq!(Command::from_str("!sweeper").unwrap(), Command::Sweeper);
        assert_eq!(Command::from_str("!clear").unwrap(), Command::Clear);
        assert_eq!(Command::from_str("!status").unwrap(), Command::Status);
        assert_eq!(Command::from_str("!help").unwrap(), Command::Help);
        assert_eq!(Command::from_str("!shutdown").unwrap(), Command::Shutdown);
    }

    /// Test `FromStr` for `Command`s errors on unknown commands.
    #[test]
    fn parse_unknown_command() {
        for command in [
            "!queu",
            "!hel",
            "!q",
            "!sudo",
            "!moo",
            "!command",
            "!a",
            "!ab",
            "!abc",
            "!",
            "!/bin/bash",
            "!/bin/sh",
            "!234",
            "!---",
            "!$%#%",
            "!@%#^",
            "!aaaaaaaa",
        ] {
            assert_eq!(
                Command::from_str(command),
                Err(CommandParseError::UnknownCommand)
            );
        }
    }

    /// Test `FromStr` for `Command::Add`.
    #[test]
    fn parse_add() {
        let command = Command::from_str("!add 2 low").unwrap();
        let expected = Command::Add((vec![2], Some(Priority::Low)));
        assert_eq!(command, expected);

        let command = Command::from_str("!add 29 1").unwrap();
        let expected = Command::Add((vec![29, 1], None));
        assert_eq!(command, expected);

        let command = Command::from_str("!add 76 high").unwrap();
        let expected = Command::Add((vec![76], Some(Priority::High)));
        assert_eq!(command, expected);

        let command = Command::from_str("!add 21 1 554 medium").unwrap();
        let expected = Command::Add((vec![21, 1, 554], Some(Priority::Medium)));
        assert_eq!(command, expected);

        let command = Command::from_str("!add 21 1 554 medium medium");
        let expected = Err(CommandParseError::UnknownParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!add 21 1 554 554 medium");
        let expected = Err(CommandParseError::DuplicateParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!add");
        let expected = Err(CommandParseError::MissingParameter);
        assert_eq!(command, expected);
    }

    /// Test `FromStr` for `Command::Add` fails with duplicate parameters.
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: DuplicateParameter")]
    fn parse_add_dup_param() {
        Command::from_str("!add 2 2 low").unwrap();
    }

    /// Test `FromStr` for `Command::Remove`.
    #[test]
    fn parse_remove() {
        let command = Command::from_str("!remove 2 1 45").unwrap();
        let expected = Command::Remove(vec![2, 1, 45]);
        assert_eq!(command, expected);

        let command = Command::from_str("!remove 52 12 45111").unwrap();
        let expected = Command::Remove(vec![52, 12, 45111]);
        assert_eq!(command, expected);

        let command = Command::from_str("!remove 25").unwrap();
        let expected = Command::Remove(vec![25]);
        assert_eq!(command, expected);

        let command = Command::from_str("!remove 25 25");
        let expected = Err(CommandParseError::DuplicateParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!remove 25 25 asdf");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!remove");
        let expected = Err(CommandParseError::MissingParameter);
        assert_eq!(command, expected);
    }

    /// Test `FromStr` for `Command::Remove` fails with duplicate parameters.
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: DuplicateParameter")]
    fn parse_remove_dup_param() {
        Command::from_str("!remove 2 2").unwrap();
    }

    /// Test [`FromStr`] for [`Command::Cancel`].
    #[test]
    fn parse_cancel() {
        let command = Command::from_str("!cancel 0 \"a b c\"").unwrap();
        let expected = Command::Cancel(NonZero::new(1).unwrap(), Some("a b c".into()));
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel 2 \"Test\"").unwrap();
        let expected = Command::Cancel(NonZero::new(2).unwrap(), Some("Test".into()));
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel 29 \"Test 2\"").unwrap();
        let expected = Command::Cancel(NonZero::new(29).unwrap(), Some("Test 2".into()));
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel -1 \"Test 2\"");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel 21 \"asdf\" \"asdf\"");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel 21 asdf");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel 21 1 \"asdf\"");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel");
        let expected = Err(CommandParseError::MissingParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!cancel \"asdf\"");
        let expected = Err(CommandParseError::IncorrectParameter);
        assert_eq!(command, expected);
    }

    /// Test `FromStr` for `Command::Agenda`.
    #[test]
    fn parse_agenda() {
        let command = Command::from_str(r#"!agenda ["hello", "world"]"#).unwrap();
        let expected = Command::Agenda(vec!["hello".into(), "world".into()]);
        assert_eq!(command, expected);

        let command = Command::from_str(r#"!agenda ["item 1", "item 2 3"]"#).unwrap();
        let expected = Command::Agenda(vec!["item 1".into(), "item 2 3".into()]);
        assert_eq!(command, expected);

        let command = Command::from_str(r#"!agenda ["item 1"]"#).unwrap();
        let expected = Command::Agenda(vec!["item 1".into()]);
        assert_eq!(command, expected);

        let command = Command::from_str("!agenda");
        let expected = Err(CommandParseError::MissingParameter);
        assert_eq!(command, expected);

        let command = Command::from_str("!agenda []");
        let expected = Err(CommandParseError::MissingParameter);
        assert_eq!(command, expected);
    }

    /// Test `FromStr` for `Command::Agenda` fails with duplicate parameters.
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: DuplicateParameter")]
    fn parse_agenda_dup_param() {
        Command::from_str(r#"!agenda ["a", "a"]"#).unwrap();
    }
}
