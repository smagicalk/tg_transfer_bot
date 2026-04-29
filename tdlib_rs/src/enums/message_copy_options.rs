#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCopyOptions {
    /// Options to be used when a message content is copied without reference to the original sender. Service messages, messages with messageInvoice, messagePaidMedia, messageGiveaway, or messageGiveawayWinners content can't be copied
    #[serde(rename(serialize = "messageCopyOptions", deserialize = "messageCopyOptions"))]
    MessageCopyOptions(crate::types::MessageCopyOptions),
}
