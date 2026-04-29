#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The transaction is a sending of a paid group call message; relevant for regular users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarTransactionTypePaidGroupCallMessageSend {
    /// Identifier of the chat that received the payment
    pub chat_id: i64,
}
