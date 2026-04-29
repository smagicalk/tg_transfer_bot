#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a reaction to a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageReaction {
    /// Type of the reaction
    pub r#type: crate::enums::ReactionType,
    /// Number of times the reaction was added
    pub total_count: i32,
    /// True, if the reaction is chosen by the current user
    pub is_chosen: bool,
    /// Identifier of the message sender used by the current user to add the reaction; may be null if unknown or the reaction isn't chosen
    pub used_sender_id: Option<crate::enums::MessageSender>,
    /// Identifiers of at most 3 recent message senders, added the reaction; available in private, basic group and supergroup chats
    pub recent_sender_ids: Vec<crate::enums::MessageSender>,
}
