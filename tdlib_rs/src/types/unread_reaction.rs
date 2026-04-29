#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about an unread reaction to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnreadReaction {
    /// Type of the reaction
    pub r#type: crate::enums::ReactionType,
    /// Identifier of the sender, added the reaction
    pub sender_id: crate::enums::MessageSender,
    /// True, if the reaction was added with a big animation
    pub is_big: bool,
}
