#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Options to be used when a message is sent
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSendOptions {
    /// Information about the suggested post; pass null if none. For messages to channel direct messages chat only. Applicable only to sendMessage and addOffer
    pub suggested_post_info: Option<crate::types::InputSuggestedPostInfo>,
    /// Pass true to disable notification for the message
    pub disable_notification: bool,
    /// Pass true if the message is sent from the background
    pub from_background: bool,
    /// Pass true if the content of the message must be protected from forwarding and saving; for bots only
    pub protect_content: bool,
    /// Pass true to allow the message to ignore regular broadcast limits for a small fee; for bots only
    pub allow_paid_broadcast: bool,
    /// The number of Telegram Stars the user agreed to pay to send the messages
    pub paid_message_star_count: i64,
    /// Pass true if the user explicitly chosen a sticker or a custom emoji from an installed sticker set; applicable only to sendMessage and sendMessageAlbum
    pub update_order_of_installed_sticker_sets: bool,
    /// Message scheduling state; pass null to send message immediately. Messages sent to a secret chat, to a chat with paid messages, to a channel direct messages chat,
    /// live location messages and self-destructing messages can't be scheduled
    pub scheduling_state: Option<crate::enums::MessageSchedulingState>,
    /// Identifier of the effect to apply to the message; pass 0 if none; applicable only to sendMessage, sendMessageAlbum in private chats and forwardMessages with one message to private chats
    #[serde_as(as = "DisplayFromStr")]
    pub effect_id: i64,
    /// Non-persistent identifier, which will be returned back in messageSendingStatePending object and can be used to match sent messages and corresponding updateNewMessage updates
    pub sending_id: i32,
    /// Pass true to get a fake message instead of actually sending them
    pub only_preview: bool,
}
