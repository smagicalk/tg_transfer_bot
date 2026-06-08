#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with an invoice from a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentInvoice {
    /// Product price
    pub price: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
