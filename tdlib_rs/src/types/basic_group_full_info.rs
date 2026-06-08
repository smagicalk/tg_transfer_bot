#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains full information about a basic group
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BasicGroupFullInfo {
    /// Chat photo; may be null if empty or unknown. If non-null, then it is the same photo as in chat.photo
    pub photo: Option<crate::types::ChatPhoto>,
    /// Group description. Updated only after the basic group is opened
    pub description: String,
    /// User identifier of the creator of the group; 0 if unknown
    pub creator_user_id: i64,
    /// Group members
    pub members: Vec<crate::types::ChatMember>,
    /// True, if non-administrators and non-bots can be hidden in responses to getSupergroupMembers and searchChatMembers for non-administrators after upgrading the basic group to a supergroup
    pub can_hide_members: bool,
    /// True, if aggressive anti-spam checks can be enabled or disabled in the supergroup after upgrading the basic group to a supergroup
    pub can_toggle_aggressive_anti_spam: bool,
    /// Primary invite link for this group; may be null. For chat administrators with can_invite_users right only. Updated only after the basic group is opened
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// List of commands of bots in the group
    pub bot_commands: Vec<crate::types::BotCommands>,
}
