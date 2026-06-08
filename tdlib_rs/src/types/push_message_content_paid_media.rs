#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with paid media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentPaidMedia {
    /// Number of Telegram Stars needed to buy access to the media in the message; 0 for pinned message
    pub star_count: i64,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
