#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a slot for chat boost
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSlot {
    /// Unique identifier of the slot
    pub slot_id: i32,
    /// Identifier of the currently boosted chat; 0 if none
    pub currently_boosted_chat_id: i64,
    /// Point in time (Unix timestamp) when the chat was boosted; 0 if none
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire
    pub expiration_date: i32,
    /// Point in time (Unix timestamp) after which the boost can be used for another chat
    pub cooldown_until_date: i32,
}
