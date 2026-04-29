#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OptionValue {
    /// Represents a boolean option
    #[serde(rename(serialize = "optionValueBoolean", deserialize = "optionValueBoolean"))]
    Boolean(crate::types::OptionValueBoolean),
    /// Represents an unknown option or an option which has a default value
    #[serde(rename(serialize = "optionValueEmpty", deserialize = "optionValueEmpty"))]
    Empty,
    /// Represents an integer option
    #[serde(rename(serialize = "optionValueInteger", deserialize = "optionValueInteger"))]
    Integer(crate::types::OptionValueInteger),
    /// Represents a string option
    #[serde(rename(serialize = "optionValueString", deserialize = "optionValueString"))]
    String(crate::types::OptionValueString),
}
