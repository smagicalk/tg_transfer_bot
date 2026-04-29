#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the are is clicked
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: crate::enums::ReactionType,
    /// Number of times the reaction was added
    pub total_count: i32,
    /// True, if reaction has a dark background
    pub is_dark: bool,
    /// True, if reaction corner is flipped
    pub is_flipped: bool,
}
