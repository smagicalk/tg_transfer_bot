#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A scope covering all members of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotCommandScopeChat {
    /// Chat identifier
    pub chat_id: i64,
}
