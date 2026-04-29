#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups,
/// ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup.
/// If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
/// check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user,
/// and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat.
/// Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeBotStartInGroup {
    /// Username of the bot
    pub bot_username: String,
    /// The parameter to be passed to sendBotStartMessage
    pub start_parameter: String,
    /// Expected administrator rights for the bot; may be null
    pub administrator_rights: Option<crate::types::ChatAdministratorRights>,
}
