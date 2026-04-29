#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a chat revenue transactions
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransaction {
    /// Cryptocurrency in which revenue is calculated
    pub cryptocurrency: String,
    /// The withdrawn amount, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub cryptocurrency_amount: i64,
    /// Type of the transaction
    pub r#type: crate::enums::ChatRevenueTransactionType,
}
