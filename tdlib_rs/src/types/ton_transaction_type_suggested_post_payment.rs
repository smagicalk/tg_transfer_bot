#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a payment for a suggested post
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TonTransactionTypeSuggestedPostPayment {
    /// Identifier of the channel chat that posted the post
    pub chat_id: i64,
}
