#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementError {
    /// Contains the description of an error in a Telegram Passport element
    #[serde(rename(serialize = "passportElementError", deserialize = "passportElementError"))]
    PassportElementError(crate::types::PassportElementError),
}
