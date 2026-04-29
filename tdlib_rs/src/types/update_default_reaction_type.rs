#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The type of default reaction has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateDefaultReactionType {
    /// The new type of the default reaction
    pub reaction_type: crate::enums::ReactionType,
}
