#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a transaction changing the amount of owned Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction
    pub id: String,
    /// The amount of added owned Telegram Stars; negative for outgoing transactions
    pub star_amount: crate::types::StarAmount,
    /// True, if the transaction is a refund of a previous transaction
    pub is_refund: bool,
    /// Point in time (Unix timestamp) when the transaction was completed
    pub date: i32,
    /// Type of the transaction
    pub r#type: crate::enums::StarTransactionType,
}
