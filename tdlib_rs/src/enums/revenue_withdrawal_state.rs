#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RevenueWithdrawalState {
    /// Withdrawal is pending
    #[serde(rename(
        serialize = "revenueWithdrawalStatePending",
        deserialize = "revenueWithdrawalStatePending"
    ))]
    Pending,
    /// Withdrawal succeeded
    #[serde(rename(
        serialize = "revenueWithdrawalStateSucceeded",
        deserialize = "revenueWithdrawalStateSucceeded"
    ))]
    Succeeded(crate::types::RevenueWithdrawalStateSucceeded),
    /// Withdrawal failed
    #[serde(rename(
        serialize = "revenueWithdrawalStateFailed",
        deserialize = "revenueWithdrawalStateFailed"
    ))]
    Failed,
}
