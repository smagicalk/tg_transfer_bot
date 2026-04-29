#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallReactionReceive {
    /// Identifier of the sender of the reaction
    pub sender_id: crate::enums::MessageSender,
    /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for reaction sending
    pub commission_per_mille: i32,
    /// The Telegram Star amount that was received by Telegram; can be negative for refunds
    pub commission_star_amount: crate::types::StarAmount,
}
