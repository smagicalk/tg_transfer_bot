#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementsWithErrors {
    /// Contains information about a Telegram Passport elements and corresponding errors
    #[serde(rename(
        serialize = "passportElementsWithErrors",
        deserialize = "passportElementsWithErrors"
    ))]
    PassportElementsWithErrors(crate::types::PassportElementsWithErrors),
}
