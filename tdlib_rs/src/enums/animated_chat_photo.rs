#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AnimatedChatPhoto {
    /// Animated variant of a chat photo in MPEG4 format
    #[serde(rename(serialize = "animatedChatPhoto", deserialize = "animatedChatPhoto"))]
    AnimatedChatPhoto(crate::types::AnimatedChatPhoto),
}
