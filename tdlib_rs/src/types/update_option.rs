#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An option changed its value
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateOption {
    /// The option name
    pub name: String,
    /// The new option value
    pub value: crate::enums::OptionValue,
}
