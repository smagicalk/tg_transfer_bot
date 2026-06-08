#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotInfo {
    /// Contains information about a bot
    #[serde(rename(serialize = "botInfo", deserialize = "botInfo"))]
    BotInfo(crate::types::BotInfo),
}
