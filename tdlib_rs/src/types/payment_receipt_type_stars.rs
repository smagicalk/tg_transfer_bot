#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The payment was done using Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentReceiptTypeStars {
    /// Number of Telegram Stars that were paid
    pub star_count: i64,
    /// Unique identifier of the transaction that can be used to dispute it
    pub transaction_id: String,
}
