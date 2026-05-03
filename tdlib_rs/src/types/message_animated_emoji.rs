#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with an animated emoji
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageAnimatedEmoji {
    /// The animated emoji
    pub animated_emoji: crate::types::AnimatedEmoji,
    /// The corresponding emoji
    pub emoji: String,
}
