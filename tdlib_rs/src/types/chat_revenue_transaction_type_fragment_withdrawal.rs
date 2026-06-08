#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a withdrawal of earnings through Fragment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeFragmentWithdrawal {
    /// Point in time (Unix timestamp) when the earnings withdrawal started
    pub withdrawal_date: i32,
    /// State of the withdrawal
    pub state: crate::enums::RevenueWithdrawalState,
}
