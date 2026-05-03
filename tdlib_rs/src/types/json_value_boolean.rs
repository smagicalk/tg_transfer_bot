#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a boolean JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueBoolean {
    /// The value
    pub value: bool,
}
