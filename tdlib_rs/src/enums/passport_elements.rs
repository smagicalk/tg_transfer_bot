#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElements {
    /// Contains information about saved Telegram Passport elements
    #[serde(rename(serialize = "passportElements", deserialize = "passportElements"))]
    PassportElements(crate::types::PassportElements),
}
