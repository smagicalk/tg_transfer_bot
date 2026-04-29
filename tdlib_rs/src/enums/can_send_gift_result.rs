#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanSendGiftResult {
    /// The gift can be sent now by the current user
    #[serde(rename(serialize = "canSendGiftResultOk", deserialize = "canSendGiftResultOk"))]
    Ok,
    /// The gift can't be sent now by the current user
    #[serde(rename(serialize = "canSendGiftResultFail", deserialize = "canSendGiftResultFail"))]
    Fail(crate::types::CanSendGiftResultFail),
}
