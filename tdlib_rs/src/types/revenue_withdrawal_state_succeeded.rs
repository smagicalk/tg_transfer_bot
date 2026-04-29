#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Withdrawal succeeded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Point in time (Unix timestamp) when the withdrawal was completed
    pub date: i32,
    /// The URL where the withdrawal transaction can be viewed
    pub url: String,
}
