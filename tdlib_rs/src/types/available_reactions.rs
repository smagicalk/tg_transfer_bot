#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of reactions that can be added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AvailableReactions {
    /// List of reactions to be shown at the top
    pub top_reactions: Vec<crate::types::AvailableReaction>,
    /// List of recently used reactions
    pub recent_reactions: Vec<crate::types::AvailableReaction>,
    /// List of popular reactions
    pub popular_reactions: Vec<crate::types::AvailableReaction>,
    /// True, if any custom emoji reaction can be added by Telegram Premium subscribers
    pub allow_custom_emoji: bool,
    /// True, if the reactions will be tags and the message can be found by them
    pub are_tags: bool,
    /// The reason why the current user can't add reactions to the message, despite some other users can; may be null if none
    pub unavailability_reason: Option<crate::enums::ReactionUnavailabilityReason>,
}
