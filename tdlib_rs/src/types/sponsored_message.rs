#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a sponsored message
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SponsoredMessage {
    /// Message identifier; unique for the chat to which the sponsored message belongs among both ordinary and sponsored messages
    pub message_id: i64,
    /// True, if the message needs to be labeled as "recommended" instead of "sponsored"
    pub is_recommended: bool,
    /// True, if the message can be reported to Telegram moderators through reportChatSponsoredMessage
    pub can_be_reported: bool,
    /// Content of the message. Currently, can be only of the types messageText, messageAnimation, messagePhoto, or messageVideo. Video messages can be viewed fullscreen
    pub content: crate::enums::MessageContent,
    /// Information about the sponsor of the message
    pub sponsor: crate::types::AdvertisementSponsor,
    /// Title of the sponsored message
    pub title: String,
    /// Text for the message action button
    pub button_text: String,
    /// Identifier of the accent color for title, button text and message background
    pub accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the message background; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub background_custom_emoji_id: i64,
    /// If non-empty, additional information about the sponsored message to be shown along with the message
    pub additional_info: String,
}
