#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputCall {
    /// A just ended call
    #[serde(rename(serialize = "inputCallDiscarded", deserialize = "inputCallDiscarded"))]
    Discarded(crate::types::InputCallDiscarded),
    /// A call from a message of the type messageCall with non-zero messageCall.unique_id
    #[serde(rename(
        serialize = "inputCallFromMessage",
        deserialize = "inputCallFromMessage"
    ))]
    FromMessage(crate::types::InputCallFromMessage),
}
