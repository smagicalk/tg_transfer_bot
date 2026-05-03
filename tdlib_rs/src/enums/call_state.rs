#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallState {
    /// The call is pending, waiting to be accepted by a user
    #[serde(rename(serialize = "callStatePending", deserialize = "callStatePending"))]
    Pending(crate::types::CallStatePending),
    /// The call has been answered and encryption keys are being exchanged
    #[serde(rename(
        serialize = "callStateExchangingKeys",
        deserialize = "callStateExchangingKeys"
    ))]
    ExchangingKeys,
    /// The call is ready to use
    #[serde(rename(serialize = "callStateReady", deserialize = "callStateReady"))]
    Ready(crate::types::CallStateReady),
    /// The call is hanging up after discardCall has been called
    #[serde(rename(serialize = "callStateHangingUp", deserialize = "callStateHangingUp"))]
    HangingUp,
    /// The call has ended successfully
    #[serde(rename(serialize = "callStateDiscarded", deserialize = "callStateDiscarded"))]
    Discarded(crate::types::CallStateDiscarded),
    /// The call has ended with an error
    #[serde(rename(serialize = "callStateError", deserialize = "callStateError"))]
    Error(crate::types::CallStateError),
}
