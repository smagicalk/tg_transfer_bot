#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a command supported by a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommand {
    /// Text of the bot command
    pub command: String,
    /// Description of the bot command
    pub description: String,
}
