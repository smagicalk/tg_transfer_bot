#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains full information about a supergroup or channel
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SupergroupFullInfo {
    /// Chat photo; may be null if empty or unknown. If non-null, then it is the same photo as in chat.photo
    pub photo: Option<crate::types::ChatPhoto>,
    /// Supergroup or channel description
    pub description: String,
    /// Number of members in the supergroup or channel; 0 if unknown
    pub member_count: i32,
    /// Number of privileged users in the supergroup or channel; 0 if unknown
    pub administrator_count: i32,
    /// Number of restricted users in the supergroup; 0 if unknown
    pub restricted_count: i32,
    /// Number of users banned from chat; 0 if unknown
    pub banned_count: i32,
    /// Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown
    pub linked_chat_id: i64,
    /// Chat identifier of a direct messages group for the channel, or a channel, for which the supergroup is the designated direct messages group; 0 if none
    pub direct_messages_chat_id: i64,
    /// Delay between consecutive sent messages for non-administrator supergroup members, in seconds
    pub slow_mode_delay: i32,
    /// Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero
    pub slow_mode_delay_expires_in: f64,
    /// True, if paid messages can be enabled in the supergroup chat; for supergroup only
    pub can_enable_paid_messages: bool,
    /// True, if paid reaction can be enabled in the channel chat; for channels only
    pub can_enable_paid_reaction: bool,
    /// True, if members of the chat can be retrieved via getSupergroupMembers or searchChatMembers
    pub can_get_members: bool,
    /// True, if non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers
    pub has_hidden_members: bool,
    /// True, if non-administrators and non-bots can be hidden in responses to getSupergroupMembers and searchChatMembers for non-administrators
    pub can_hide_members: bool,
    /// True, if the supergroup sticker set can be changed
    pub can_set_sticker_set: bool,
    /// True, if the supergroup location can be changed
    pub can_set_location: bool,
    /// True, if the supergroup or channel statistics are available
    pub can_get_statistics: bool,
    /// True, if the supergroup or channel revenue statistics are available
    pub can_get_revenue_statistics: bool,
    /// True, if the supergroup or channel Telegram Star revenue statistics are available
    pub can_get_star_revenue_statistics: bool,
    /// True, if the user can send a gift to the supergroup or channel using sendGift or transferGift
    pub can_send_gift: bool,
    /// True, if aggressive anti-spam checks can be enabled or disabled in the supergroup
    pub can_toggle_aggressive_anti_spam: bool,
    /// True, if new chat members will have access to old messages. In public, discussion, of forum groups and all channels, old messages are always available,
    /// so this option affects only private non-forum supergroups without a linked chat. The value of this field is only available to chat administrators
    pub is_all_history_available: bool,
    /// True, if the chat can have sponsored messages. The value of this field is only available to the owner of the chat
    pub can_have_sponsored_messages: bool,
    /// True, if aggressive anti-spam checks are enabled in the supergroup. The value of this field is only available to chat administrators
    pub has_aggressive_anti_spam_enabled: bool,
    /// True, if paid media can be sent and forwarded to the channel chat; for channels only
    pub has_paid_media_allowed: bool,
    /// True, if the supergroup or channel has pinned stories
    pub has_pinned_stories: bool,
    /// Number of saved to profile gifts for channels without can_post_messages administrator right, otherwise, the total number of received gifts
    pub gift_count: i32,
    /// Number of times the current user boosted the supergroup or channel
    pub my_boost_count: i32,
    /// Number of times the supergroup must be boosted by a user to ignore slow mode and chat permission restrictions; 0 if unspecified
    pub unrestrict_boost_count: i32,
    /// Number of Telegram Stars that must be paid by the current user for each sent message to the supergroup
    pub outgoing_paid_message_star_count: i64,
    /// Identifier of the supergroup sticker set that must be shown before user sticker sets; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub sticker_set_id: i64,
    /// Identifier of the custom emoji sticker set that can be used in the supergroup without Telegram Premium subscription; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_sticker_set_id: i64,
    /// Location to which the supergroup is connected; may be null if none
    pub location: Option<crate::types::ChatLocation>,
    /// Primary invite link for the chat; may be null. For chat administrators with can_invite_users right only
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// List of commands of bots in the group
    pub bot_commands: Vec<crate::types::BotCommands>,
    /// Information about verification status of the supergroup or the channel provided by a bot; may be null if none or unknown
    pub bot_verification: Option<crate::types::BotVerification>,
    /// The main tab chosen by the administrators of the channel; may be null if not chosen manually
    pub main_profile_tab: Option<crate::enums::ProfileTab>,
    /// Identifier of the basic group from which supergroup was upgraded; 0 if none
    pub upgraded_from_basic_group_id: i64,
    /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none
    pub upgraded_from_max_message_id: i64,
}
