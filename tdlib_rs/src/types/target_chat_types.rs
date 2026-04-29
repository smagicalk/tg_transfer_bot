#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes allowed types for the target chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TargetChatTypes {
    /// True, if private chats with ordinary users are allowed
    pub allow_user_chats: bool,
    /// True, if private chats with other bots are allowed
    pub allow_bot_chats: bool,
    /// True, if basic group and supergroup chats are allowed
    pub allow_group_chats: bool,
    /// True, if channel chats are allowed
    pub allow_channel_chats: bool,
}
