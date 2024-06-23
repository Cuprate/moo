//! TODO

//---------------------------------------------------------------------------------------------------- Use

use serde::{Deserialize, Serialize};
use strum::{
    AsRefStr, Display, EnumCount, EnumIs, EnumIter, FromRepr, IntoStaticStr, VariantNames,
};

//---------------------------------------------------------------------------------------------------- Matrix IDs
/// TODO
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    EnumCount,
    EnumIs,
    EnumIter,
    FromRepr,
    IntoStaticStr,
    VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Priority {
    /// TODO
    #[serde(alias = "Low", alias = "LOW", alias = "0")]
    Low = 0,

    /// TODO
    #[default]
    #[serde(alias = "Medium", alias = "MEDIUM", alias = "1")]
    Medium = 1,

    /// TODO
    #[serde(alias = "High", alias = "HIGH", alias = "2")]
    High = 2,

    /// TODO
    #[serde(alias = "Critical", alias = "CRITICAL", alias = "3")]
    Critical = 3,
}
