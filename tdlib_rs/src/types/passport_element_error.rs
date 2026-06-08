#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains the description of an error in a Telegram Passport element
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PassportElementError {
    /// Type of the Telegram Passport element which has the error
    pub r#type: crate::enums::PassportElementType,
    /// Error message
    pub message: String,
    /// Error source
    pub source: crate::enums::PassportElementErrorSource,
}
