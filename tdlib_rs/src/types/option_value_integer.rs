#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents an integer option
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OptionValueInteger {
    /// The value of the option
    #[serde_as(as = "DisplayFromStr")]
    pub value: i64,
}
