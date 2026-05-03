#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A forward of the story as a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeForward {
    /// The message with story forward
    pub message: crate::types::Message,
}
