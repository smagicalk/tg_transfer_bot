#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBusinessBotTransferSend {
    /// Identifier of the bot that received Telegram Stars
    pub user_id: i64,
}
