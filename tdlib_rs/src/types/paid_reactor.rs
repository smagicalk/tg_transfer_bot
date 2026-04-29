#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a user who added paid reactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidReactor {
    /// Identifier of the user or chat that added the reactions; may be null for anonymous reactors that aren't the current user
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Number of Telegram Stars added
    pub star_count: i64,
    /// True, if the reactor is one of the most active reactors; may be false if the reactor is the current user
    pub is_top: bool,
    /// True, if the paid reaction was added by the current user
    pub is_me: bool,
    /// True, if the reactor is anonymous
    pub is_anonymous: bool,
}
