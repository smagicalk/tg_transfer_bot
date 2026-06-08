#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a refund for failed withdrawal of earnings through Fragment
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeFragmentRefund {
    /// Point in time (Unix timestamp) when the transaction was refunded
    pub refund_date: i32,
}
