#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeAffiliateProgramCommission {
    /// Identifier of the chat that created the affiliate program
    pub chat_id: i64,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
    pub commission_per_mille: i32,
}
