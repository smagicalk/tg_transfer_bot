#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeSuggestedPostPaymentReceive {
    /// Identifier of the user who paid for the suggested post
    pub user_id: i64,
}
