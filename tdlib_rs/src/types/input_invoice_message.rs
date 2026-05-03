#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An invoice from a message of the type messageInvoice or paid media purchase from messagePaidMedia
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputInvoiceMessage {
    /// Chat identifier of the message
    pub chat_id: i64,
    /// Message identifier. Use messageProperties.can_be_paid to check whether the message can be used in the method
    pub message_id: i64,
}
