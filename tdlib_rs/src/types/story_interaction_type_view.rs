#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A view of the story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeView {
    /// Type of the reaction that was chosen by the viewer; may be null if none
    pub chosen_reaction_type: Option<crate::enums::ReactionType>,
}
