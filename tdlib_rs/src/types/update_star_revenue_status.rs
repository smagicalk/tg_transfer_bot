#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStarRevenueStatus {
    /// Identifier of the owner of the Telegram Stars
    pub owner_id: crate::enums::MessageSender,
    /// New Telegram Star revenue status
    pub status: crate::types::StarRevenueStatus,
}
