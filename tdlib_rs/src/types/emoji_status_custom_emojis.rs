#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of custom emoji identifiers for emoji statuses
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiStatusCustomEmojis {
    /// The list of custom emoji identifiers
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub custom_emoji_ids: Vec<i64>,
}
