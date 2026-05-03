#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AnimatedEmoji {
    /// Describes an animated or custom representation of an emoji
    #[serde(rename(serialize = "animatedEmoji", deserialize = "animatedEmoji"))]
    AnimatedEmoji(crate::types::AnimatedEmoji),
}
