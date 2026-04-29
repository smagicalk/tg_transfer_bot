#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionType {
    /// A reaction with an emoji
    #[serde(rename(serialize = "reactionTypeEmoji", deserialize = "reactionTypeEmoji"))]
    Emoji(crate::types::ReactionTypeEmoji),
    /// A reaction with a custom emoji
    #[serde(rename(serialize = "reactionTypeCustomEmoji", deserialize = "reactionTypeCustomEmoji"))]
    CustomEmoji(crate::types::ReactionTypeCustomEmoji),
    /// The paid reaction in a channel chat
    #[serde(rename(serialize = "reactionTypePaid", deserialize = "reactionTypePaid"))]
    Paid,
}
