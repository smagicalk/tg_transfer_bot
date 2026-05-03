#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes rights of a business bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessBotRights {
    /// True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    pub can_reply: bool,
    /// True, if the bot can mark incoming private messages as read
    pub can_read_messages: bool,
    /// True, if the bot can delete sent messages
    pub can_delete_sent_messages: bool,
    /// True, if the bot can delete any message
    pub can_delete_all_messages: bool,
    /// True, if the bot can edit name of the business account
    pub can_edit_name: bool,
    /// True, if the bot can edit bio of the business account
    pub can_edit_bio: bool,
    /// True, if the bot can edit profile photo of the business account
    pub can_edit_profile_photo: bool,
    /// True, if the bot can edit username of the business account
    pub can_edit_username: bool,
    /// True, if the bot can view gifts and Telegram Star amount owned by the business account
    pub can_view_gifts_and_stars: bool,
    /// True, if the bot can sell regular gifts received by the business account
    pub can_sell_gifts: bool,
    /// True, if the bot can change gift receiving settings of the business account
    pub can_change_gift_settings: bool,
    /// True, if the bot can transfer and upgrade gifts received by the business account
    pub can_transfer_and_upgrade_gifts: bool,
    /// True, if the bot can transfer Telegram Stars received by the business account to account of the bot, or use them to upgrade and transfer gifts
    pub can_transfer_stars: bool,
    /// True, if the bot can post, edit and delete stories
    pub can_manage_stories: bool,
}
