#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat available reactions were changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatEventAvailableReactionsChanged {
    /// Previous chat available reactions
    pub old_available_reactions: crate::enums::ChatAvailableReactions,
    /// New chat available reactions
    pub new_available_reactions: crate::enums::ChatAvailableReactions,
}
