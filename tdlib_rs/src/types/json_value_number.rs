#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a numeric JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueNumber {
    /// The value
    pub value: f64,
}
