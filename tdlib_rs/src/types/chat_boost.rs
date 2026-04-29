#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a boost applied to a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub id: String,
    /// The number of identical boosts applied
    pub count: i32,
    /// Source of the boost
    pub source: crate::enums::ChatBoostSource,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire
    pub expiration_date: i32,
}
