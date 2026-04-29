#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a level of features for a message sent in a live story group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallMessageLevel {
    /// The minimum number of Telegram Stars required to get features of the level
    pub min_star_count: i64,
    /// The amount of time the message of this level will be pinned, in seconds
    pub pin_duration: i32,
    /// The maximum allowed length of the message text
    pub max_text_length: i32,
    /// The maximum allowed number of custom emoji in the message text
    pub max_custom_emoji_count: i32,
    /// The first color used to show the message text in the RGB format
    pub first_color: i32,
    /// The second color used to show the message text in the RGB format
    pub second_color: i32,
    /// Background color for the message the RGB format
    pub background_color: i32,
}
