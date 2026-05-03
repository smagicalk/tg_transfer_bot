#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputGroupCall {
    /// The group call is accessible through a link
    #[serde(rename(serialize = "inputGroupCallLink", deserialize = "inputGroupCallLink"))]
    Link(crate::types::InputGroupCallLink),
    /// The group call is accessible through a message of the type messageGroupCall
    #[serde(rename(
        serialize = "inputGroupCallMessage",
        deserialize = "inputGroupCallMessage"
    ))]
    Message(crate::types::InputGroupCallMessage),
}
