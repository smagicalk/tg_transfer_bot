#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat has an active live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ActiveStoryStateLive {
    /// Identifier of the active live story
    pub story_id: i32,
}
