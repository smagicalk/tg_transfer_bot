#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains privacy settings for chats with non-contacts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NewChatPrivacySettings {
    /// True, if non-contacts users are able to write first to the current user. Telegram Premium subscribers are able to write first regardless of this setting
    pub allow_new_chats_from_unknown_users: bool,
    /// Number of Telegram Stars that must be paid for every incoming private message by non-contacts; 0-getOption("paid_message_star_count_max").
    /// If positive, then allow_new_chats_from_unknown_users must be true. The current user will receive getOption("paid_message_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for message sending.
    /// Can be positive, only if getOption("can_enable_paid_messages") is true
    pub incoming_paid_message_star_count: i64,
}
