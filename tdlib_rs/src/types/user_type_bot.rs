#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A bot (see https:core.telegram.org/bots)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserTypeBot {
    /// True, if the bot is owned by the current user and can be edited using the methods toggleBotUsernameIsActive, reorderBotActiveUsernames, setBotProfilePhoto, setBotName, setBotInfoDescription, and setBotInfoShortDescription
    pub can_be_edited: bool,
    /// True, if the bot can be invited to basic group and supergroup chats
    pub can_join_groups: bool,
    /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages
    pub can_read_all_group_messages: bool,
    /// True, if the bot has the main Web App
    pub has_main_web_app: bool,
    /// True, if the bot has topics
    pub has_topics: bool,
    /// True, if users can create and delete topics in the chat with the bot
    pub allows_users_to_create_topics: bool,
    /// True, if the bot supports inline queries
    pub is_inline: bool,
    /// Placeholder for inline queries (displayed on the application input field)
    pub inline_query_placeholder: String,
    /// True, if the location of the user is expected to be sent with every inline query to this bot
    pub need_location: bool,
    /// True, if the bot supports connection to Telegram Business accounts
    pub can_connect_to_business: bool,
    /// True, if the bot can be added to attachment or side menu
    pub can_be_added_to_attachment_menu: bool,
    /// The number of recently active users of the bot
    pub active_user_count: i32,
}
