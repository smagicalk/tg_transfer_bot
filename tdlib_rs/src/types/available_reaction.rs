#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents an available reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AvailableReaction {
    /// Type of the reaction
    pub r#type: crate::enums::ReactionType,
    /// True, if Telegram Premium is needed to send the reaction
    pub needs_premium: bool,
}
