#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The message will be sent when the video in the message is converted and optimized; can be used only by the server
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSchedulingStateSendWhenVideoProcessed {
    /// Approximate point in time (Unix timestamp) when the message is expected to be sent
    pub send_date: i32,
}
