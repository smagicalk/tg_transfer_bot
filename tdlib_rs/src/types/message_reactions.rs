#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of reactions added to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReactions {
    /// List of added reactions
    pub reactions: Vec<crate::types::MessageReaction>,
    /// True, if the reactions are tags and Telegram Premium users can filter messages by them
    pub are_tags: bool,
    /// Information about top users that added the paid reaction
    pub paid_reactors: Vec<crate::types::PaidReactor>,
    /// True, if the list of added reactions is available using getMessageAddedReactions
    pub can_get_added_reactions: bool,
}
