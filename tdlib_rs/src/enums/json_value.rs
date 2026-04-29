#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum JsonValue {
    /// Represents a null JSON value
    #[serde(rename(serialize = "jsonValueNull", deserialize = "jsonValueNull"))]
    Null,
    /// Represents a boolean JSON value
    #[serde(rename(serialize = "jsonValueBoolean", deserialize = "jsonValueBoolean"))]
    Boolean(crate::types::JsonValueBoolean),
    /// Represents a numeric JSON value
    #[serde(rename(serialize = "jsonValueNumber", deserialize = "jsonValueNumber"))]
    Number(crate::types::JsonValueNumber),
    /// Represents a string JSON value
    #[serde(rename(serialize = "jsonValueString", deserialize = "jsonValueString"))]
    String(crate::types::JsonValueString),
    /// Represents a JSON array
    #[serde(rename(serialize = "jsonValueArray", deserialize = "jsonValueArray"))]
    Array(crate::types::JsonValueArray),
    /// Represents a JSON object
    #[serde(rename(serialize = "jsonValueObject", deserialize = "jsonValueObject"))]
    Object(crate::types::JsonValueObject),
}
