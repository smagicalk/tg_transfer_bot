#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents one member of a JSON object
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JsonObjectMember {
    /// Member's key
    pub key: String,
    /// Member's value
    pub value: crate::enums::JsonValue,
}
