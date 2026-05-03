#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A date and time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypeDateTime {
    /// Point in time (Unix timestamp) representing the date and time
    pub unix_time: i32,
    /// Date and time formatting type; may be null if none and the original text must not be changed
    pub formatting_type: Option<crate::enums::DateTimeFormattingType>,
}
