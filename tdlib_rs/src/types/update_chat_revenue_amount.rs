#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatRevenueAmount {
    /// Identifier of the chat
    pub chat_id: i64,
    /// New amount of earned revenue
    pub revenue_amount: crate::types::ChatRevenueAmount,
}
