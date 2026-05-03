#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new paid reaction was received in a live story group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewGroupCallPaidReaction {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// Identifier of the sender of the reaction
    pub sender_id: crate::enums::MessageSender,
    /// The number of Telegram Stars that were paid to send the reaction
    pub star_count: i64,
}
