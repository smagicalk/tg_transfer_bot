#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a boolean option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueBoolean {
    /// The value of the option
    pub value: bool,
}
