#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidMessageReceive {
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// Number of received paid messages
    pub message_count: i32,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for message sending
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
}
