#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a TDLib internal log verbosity level
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogVerbosityLevel {
    /// Log verbosity level
    pub verbosity_level: i32,
}
