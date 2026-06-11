#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElementError {
    /// Contains the description of an error in a Telegram Passport element; for bots only
    #[serde(rename(
        serialize = "inputPassportElementError",
        deserialize = "inputPassportElementError"
    ))]
    InputPassportElementError(crate::types::InputPassportElementError),
}
