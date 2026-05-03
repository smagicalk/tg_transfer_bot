#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeBusinessBotTransferReceive {
    /// Identifier of the user who sent Telegram Stars
    pub user_id: i64,
}
