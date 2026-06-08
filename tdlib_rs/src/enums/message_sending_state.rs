#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSendingState {
    /// The message is being sent now, but has not yet been delivered to the server
    #[serde(rename(
        serialize = "messageSendingStatePending",
        deserialize = "messageSendingStatePending"
    ))]
    Pending(crate::types::MessageSendingStatePending),
    /// The message failed to be sent
    #[serde(rename(
        serialize = "messageSendingStateFailed",
        deserialize = "messageSendingStateFailed"
    ))]
    Failed(crate::types::MessageSendingStateFailed),
}
