#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessagePaidMedia {
    /// The number of Telegram Stars that must be paid to see the media; 1-getOption("paid_media_message_star_count_max")
    pub star_count: i64,
    /// The content of the paid media
    pub paid_media: Vec<crate::types::InputPaidMedia>,
    /// Message caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// True, if the caption must be shown above the media; otherwise, the caption must be shown below the media; not supported in secret chats
    pub show_caption_above_media: bool,
    /// Bot-provided data for the paid media; bots only
    pub payload: String,
}
