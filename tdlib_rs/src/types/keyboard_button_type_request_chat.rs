#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestChat {
    /// Unique button identifier
    pub id: i32,
    /// True, if the chat must be a channel; otherwise, a basic group or a supergroup chat is shared
    pub chat_is_channel: bool,
    /// True, if the chat must or must not be a forum supergroup
    pub restrict_chat_is_forum: bool,
    /// True, if the chat must be a forum supergroup; otherwise, the chat must not be a forum supergroup. Ignored if restrict_chat_is_forum is false
    pub chat_is_forum: bool,
    /// True, if the chat must or must not have a username
    pub restrict_chat_has_username: bool,
    /// True, if the chat must have a username; otherwise, the chat must not have a username. Ignored if restrict_chat_has_username is false
    pub chat_has_username: bool,
    /// True, if the chat must be created by the current user
    pub chat_is_created: bool,
    /// Expected user administrator rights in the chat; may be null if they aren't restricted
    pub user_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Expected bot administrator rights in the chat; may be null if they aren't restricted
    pub bot_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// True, if the bot must be a member of the chat; for basic group and supergroup chats only
    pub bot_is_member: bool,
    /// Pass true to request title of the chat; bots only
    pub request_title: bool,
    /// Pass true to request username of the chat; bots only
    pub request_username: bool,
    /// Pass true to request photo of the chat; bots only
    pub request_photo: bool,
}
