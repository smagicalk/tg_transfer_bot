#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of bot commands
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommands {
    /// Bot's user identifier
    pub bot_user_id: i64,
    /// List of bot commands
    pub commands: Vec<crate::types::BotCommand>,
}
