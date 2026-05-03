#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotVerification {
    /// Describes verification status provided by a bot
    #[serde(rename(serialize = "botVerification", deserialize = "botVerification"))]
    BotVerification(crate::types::BotVerification),
}
