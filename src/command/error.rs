//! TODO

//---------------------------------------------------------------------------------------------------- Use

//---------------------------------------------------------------------------------------------------- CommandParseError
/// TODO
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum CommandParseError {
    /// TODO
    #[error("unknown command")]
    UnknownCommand,

    /// TODO
    #[error("missing command")]
    MissingCommand,

    /// TODO
    #[error("unknown parameter")]
    UnknownParameter,

    /// TODO
    #[error("missing parameter(s)")]
    MissingParameter,

    /// TODO
    #[error("incorrect parameter(s)")]
    IncorrectParameter,

    /// TODO
    #[error("duplicate parameter(s)")]
    DuplicateParameter,
}
