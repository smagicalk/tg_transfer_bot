#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a message that can be used for quick reply
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct QuickReplyMessage {
    /// Unique message identifier among all quick replies
    pub id: i64,
    /// The sending state of the message; may be null if the message isn't being sent and didn't fail to be sent
    pub sending_state: Option<crate::enums::MessageSendingState>,
    /// True, if the message can be edited
    pub can_be_edited: bool,
    /// The identifier of the quick reply message to which the message replies; 0 if none
    pub reply_to_message_id: i64,
    /// If non-zero, the user identifier of the bot through which this message was sent
    pub via_bot_user_id: i64,
    /// Unique identifier of an album this message belongs to; 0 if none. Only audios, documents, photos and videos can be grouped together in albums
    #[serde_as(as = "DisplayFromStr")]
    pub media_album_id: i64,
    /// Content of the message
    pub content: crate::enums::MessageContent,
    /// Inline keyboard reply markup for the message; may be null if none
    pub reply_markup: Option<crate::enums::ReplyMarkup>,
}
