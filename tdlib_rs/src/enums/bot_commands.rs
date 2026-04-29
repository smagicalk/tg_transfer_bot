#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommands {
    /// Contains a list of bot commands
    #[serde(rename(serialize = "botCommands", deserialize = "botCommands"))]
    BotCommands(crate::types::BotCommands),
}
