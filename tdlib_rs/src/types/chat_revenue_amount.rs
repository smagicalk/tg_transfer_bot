#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about revenue earned from sponsored messages in a chat
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueAmount {
    /// Cryptocurrency in which revenue is calculated
    pub cryptocurrency: String,
    /// Total amount of the cryptocurrency earned, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: i64,
    /// Amount of the cryptocurrency that isn't withdrawn yet, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub balance_amount: i64,
    /// Amount of the cryptocurrency available for withdrawal, in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub available_amount: i64,
    /// True, if Telegram Stars can be withdrawn now or later
    pub withdrawal_enabled: bool,
}
