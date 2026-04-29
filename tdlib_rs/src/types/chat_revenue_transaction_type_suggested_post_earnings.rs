#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes earnings from a published suggested post
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatRevenueTransactionTypeSuggestedPostEarnings {
    /// Identifier of the user who paid for the suggested post
    pub user_id: i64,
}
