#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommand {
    /// Represents a command supported by a bot
    #[serde(rename(serialize = "botCommand", deserialize = "botCommand"))]
    BotCommand(crate::types::BotCommand),
}
