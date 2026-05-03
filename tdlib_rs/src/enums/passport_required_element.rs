#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportRequiredElement {
    /// Contains a description of the required Telegram Passport element that was requested by a service
    #[serde(rename(
        serialize = "passportRequiredElement",
        deserialize = "passportRequiredElement"
    ))]
    PassportRequiredElement(crate::types::PassportRequiredElement),
}
