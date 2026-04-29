#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a payment for Telegram API usage; relevant for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeTelegramApiUsage {
    /// The number of billed requests
    pub request_count: i32,
}
