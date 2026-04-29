#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a string option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueString {
    /// The value of the option
    pub value: String,
}
