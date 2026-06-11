#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementError {
    /// Type of Telegram Passport element that has the error
    pub r#type: crate::enums::PassportElementType,
    /// Error message
    pub message: String,
    /// Error source
    pub source: crate::enums::InputPassportElementErrorSource,
}
