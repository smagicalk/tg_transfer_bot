#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Toncoins were gifted to a user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageGiftedTon {
    /// The identifier of a user who gifted Toncoins; 0 if the gift was anonymous or is outgoing
    pub gifter_user_id: i64,
    /// The identifier of a user who received Toncoins; 0 if the gift is incoming
    pub receiver_user_id: i64,
    /// The received Toncoin amount, in the smallest units of the cryptocurrency
    pub ton_amount: i64,
    /// Identifier of the transaction for Toncoin credit; for receiver only
    pub transaction_id: String,
    /// A sticker to be shown in the message; may be null if unknown
    pub sticker: Option<crate::types::Sticker>,
}
