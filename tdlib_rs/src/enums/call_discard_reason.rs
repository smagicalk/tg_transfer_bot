#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallDiscardReason {
    /// The call wasn't discarded, or the reason is unknown
    #[serde(rename(
        serialize = "callDiscardReasonEmpty",
        deserialize = "callDiscardReasonEmpty"
    ))]
    Empty,
    /// The call was ended before the conversation started. It was canceled by the caller or missed by the other party
    #[serde(rename(
        serialize = "callDiscardReasonMissed",
        deserialize = "callDiscardReasonMissed"
    ))]
    Missed,
    /// The call was ended before the conversation started. It was declined by the other party
    #[serde(rename(
        serialize = "callDiscardReasonDeclined",
        deserialize = "callDiscardReasonDeclined"
    ))]
    Declined,
    /// The call was ended during the conversation because the users were disconnected
    #[serde(rename(
        serialize = "callDiscardReasonDisconnected",
        deserialize = "callDiscardReasonDisconnected"
    ))]
    Disconnected,
    /// The call was ended because one of the parties hung up
    #[serde(rename(
        serialize = "callDiscardReasonHungUp",
        deserialize = "callDiscardReasonHungUp"
    ))]
    HungUp,
    /// The call was ended because it has been upgraded to a group call
    #[serde(rename(
        serialize = "callDiscardReasonUpgradeToGroupCall",
        deserialize = "callDiscardReasonUpgradeToGroupCall"
    ))]
    UpgradeToGroupCall(crate::types::CallDiscardReasonUpgradeToGroupCall),
}
