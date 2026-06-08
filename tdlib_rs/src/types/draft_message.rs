#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a message draft
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DraftMessage {
    /// Information about the message to be replied; inputMessageReplyToStory is unsupported; may be null if none
    pub reply_to: Option<crate::enums::InputMessageReplyTo>,
    /// Point in time (Unix timestamp) when the draft was created
    pub date: i32,
    /// Content of the message draft; must be of the type inputMessageText, inputMessageVideoNote, or inputMessageVoiceNote
    pub input_message_text: crate::enums::InputMessageContent,
    /// Identifier of the effect to apply to the message when it is sent; 0 if none
    #[serde_as(as = "DisplayFromStr")]
    pub effect_id: i64,
    /// Information about the suggested post; may be null if none
    pub suggested_post_info: Option<crate::types::InputSuggestedPostInfo>,
}
