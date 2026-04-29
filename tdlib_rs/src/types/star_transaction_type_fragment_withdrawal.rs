#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypeFragmentWithdrawal {
    /// State of the withdrawal; may be null for refunds from Fragment
    pub withdrawal_state: Option<crate::enums::RevenueWithdrawalState>,
}
