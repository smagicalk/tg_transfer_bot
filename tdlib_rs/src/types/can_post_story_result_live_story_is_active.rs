#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user or the chat has an active live story. The live story must be deleted first
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultLiveStoryIsActive {
    /// Identifier of the active live story
    pub story_id: i32,
}
