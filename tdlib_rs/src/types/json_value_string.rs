#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a string JSON value
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonValueString {
    /// The value
    pub value: String,
}
