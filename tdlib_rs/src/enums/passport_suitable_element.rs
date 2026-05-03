#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportSuitableElement {
    /// Contains information about a Telegram Passport element that was requested by a service
    #[serde(rename(
        serialize = "passportSuitableElement",
        deserialize = "passportSuitableElement"
    ))]
    PassportSuitableElement(crate::types::PassportSuitableElement),
}
