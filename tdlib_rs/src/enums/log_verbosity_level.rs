#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogVerbosityLevel {
    /// Contains a TDLib internal log verbosity level
    #[serde(rename(serialize = "logVerbosityLevel", deserialize = "logVerbosityLevel"))]
    LogVerbosityLevel(crate::types::LogVerbosityLevel),
}
