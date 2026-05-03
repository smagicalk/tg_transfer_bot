#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A reaction with a custom emoji
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReactionTypeCustomEmoji {
    /// Unique identifier of the custom emoji
    #[serde_as(as = "DisplayFromStr")]
    pub custom_emoji_id: i64,
}
