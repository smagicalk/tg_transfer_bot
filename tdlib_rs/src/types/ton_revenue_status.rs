#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about Toncoins earned by the current user
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonRevenueStatus {
    /// Total Toncoin amount earned; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub total_amount: i64,
    /// The Toncoin amount that isn't withdrawn yet; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub balance_amount: i64,
    /// The Toncoin amount that is available for withdrawal; in the smallest units of the cryptocurrency
    #[serde_as(as = "DisplayFromStr")]
    pub available_amount: i64,
    /// True, if Toncoins can be withdrawn
    pub withdrawal_enabled: bool,
}
