#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An area pointing to a suggested reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: crate::enums::ReactionType,
    /// True, if reaction has a dark background
    pub is_dark: bool,
    /// True, if reaction corner is flipped
    pub is_flipped: bool,
}
