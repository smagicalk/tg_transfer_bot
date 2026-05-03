#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSchedulingState {
    /// The message will be sent at the specified date
    #[serde(rename(
        serialize = "messageSchedulingStateSendAtDate",
        deserialize = "messageSchedulingStateSendAtDate"
    ))]
    SendAtDate(crate::types::MessageSchedulingStateSendAtDate),
    /// The message will be sent when the other user is online. Applicable to private chats only and when the exact online status of the other user is known
    #[serde(rename(
        serialize = "messageSchedulingStateSendWhenOnline",
        deserialize = "messageSchedulingStateSendWhenOnline"
    ))]
    SendWhenOnline,
    /// The message will be sent when the video in the message is converted and optimized; can be used only by the server
    #[serde(rename(
        serialize = "messageSchedulingStateSendWhenVideoProcessed",
        deserialize = "messageSchedulingStateSendWhenVideoProcessed"
    ))]
    SendWhenVideoProcessed(crate::types::MessageSchedulingStateSendWhenVideoProcessed),
}
