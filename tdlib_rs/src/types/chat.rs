#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Chat {
    /// Chat unique identifier
    pub id: i64,
    /// Type of the chat
    pub r#type: crate::enums::ChatType,
    /// Chat title
    pub title: String,
    /// Chat photo; may be null
    pub photo: Option<crate::types::ChatPhotoInfo>,
    /// Identifier of the accent color for message sender name, and backgrounds of chat photo, reply header, and link preview
    pub accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the reply header and link preview background for messages sent by the chat; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// Color scheme based on an upgraded gift to be used for the chat instead of accent_color_id and background_custom_emoji_id; may be null if none
    pub upgraded_gift_colors: Option<crate::types::UpgradedGiftColors>,
    /// Identifier of the profile accent color for the chat's profile; -1 if none
    pub profile_accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the background of the chat's profile; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub profile_background_custom_emoji_id: i64,
    /// Actions that non-administrator chat members are allowed to take in the chat
    pub permissions: crate::types::ChatPermissions,
    /// Last message in the chat; may be null if none or unknown
    pub last_message: Option<crate::types::Message>,
    /// Positions of the chat in chat lists
    pub positions: Vec<crate::types::ChatPosition>,
    /// Chat lists to which the chat belongs. A chat can have a non-zero position in a chat list even if it doesn't belong to the chat list and have no position in a chat list even if it belongs to the chat list
    pub chat_lists: Vec<crate::enums::ChatList>,
    /// Identifier of a user or chat that is selected to send messages in the chat; may be null if the user can't change message sender
    pub message_sender_id: Option<crate::enums::MessageSender>,
    /// Block list to which the chat is added; may be null if none
    pub block_list: Option<crate::enums::BlockList>,
    /// True, if chat content can't be saved locally, forwarded, or copied
    pub has_protected_content: bool,
    /// True, if translation of all messages in the chat must be suggested to the user
    pub is_translatable: bool,
    /// True, if the chat is marked as unread
    pub is_marked_as_unread: bool,
    /// True, if the chat is a forum supergroup that must be shown in the "View as topics" mode, or Saved Messages chat that must be shown in the "View as chats"
    pub view_as_topics: bool,
    /// True, if the chat has scheduled messages
    pub has_scheduled_messages: bool,
    /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages
    pub can_be_deleted_only_for_self: bool,
    /// True, if the chat messages can be deleted for all users
    pub can_be_deleted_for_all_users: bool,
    /// True, if the chat can be reported to Telegram moderators through reportChat or reportChatPhoto
    pub can_be_reported: bool,
    /// Default value of the disable_notification parameter, used when a message is sent to the chat
    pub default_disable_notification: bool,
    /// Number of unread messages in the chat
    pub unread_count: i32,
    /// Identifier of the last read incoming message
    pub last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing message
    pub last_read_outbox_message_id: i64,
    /// Number of unread messages with a mention/reply in the chat
    pub unread_mention_count: i32,
    /// Number of messages with unread reactions in the chat
    pub unread_reaction_count: i32,
    /// Notification settings for the chat
    pub notification_settings: crate::types::ChatNotificationSettings,
    /// Types of reaction, available in the chat
    pub available_reactions: crate::enums::ChatAvailableReactions,
    /// Current message auto-delete or self-destruct timer setting for the chat, in seconds; 0 if disabled. Self-destruct timer in secret chats starts after the message or its content is viewed. Auto-delete timer in other chats starts from the send date
    pub message_auto_delete_time: i32,
    /// Emoji status to be shown along with chat title; may be null
    pub emoji_status: Option<crate::types::EmojiStatus>,
    /// Background set for the chat; may be null if none
    pub background: Option<crate::types::ChatBackground>,
    /// Theme set for the chat; may be null if none
    pub theme: Option<crate::enums::ChatTheme>,
    /// Information about actions which must be possible to do through the chat action bar; may be null if none
    pub action_bar: Option<crate::enums::ChatActionBar>,
    /// Information about bar for managing a business bot in the chat; may be null if none
    pub business_bot_manage_bar: Option<crate::types::BusinessBotManageBar>,
    /// Information about video chat of the chat
    pub video_chat: crate::types::VideoChat,
    /// Information about pending join requests; may be null if none
    pub pending_join_requests: Option<crate::types::ChatJoinRequestsInfo>,
    /// Identifier of the message from which reply markup needs to be used; 0 if there is no reply markup in the chat
    pub reply_markup_message_id: i64,
    /// A draft of a message in the chat; may be null if none
    pub draft_message: Option<crate::types::DraftMessage>,
    /// Application-specific data associated with the chat. (For example, the chat scroll position or local chat notification settings can be stored here.) Persistent if the message database is used
    pub client_data: String,
}
