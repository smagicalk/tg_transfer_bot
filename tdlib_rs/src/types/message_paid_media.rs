#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with paid media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePaidMedia {
    /// Number of Telegram Stars needed to buy access to the media in the message
    pub star_count: i64,
    /// Information about the media
    pub media: Vec<crate::enums::PaidMedia>,
    /// Media caption
    pub caption: crate::types::FormattedText,
    /// True, if the caption must be shown above the media; otherwise, the caption must be shown below the media
    pub show_caption_above_media: bool,
}
