#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SecretChatState {
    /// The secret chat is not yet created; waiting for the other user to get online
    #[serde(rename(serialize = "secretChatStatePending", deserialize = "secretChatStatePending"))]
    Pending,
    /// The secret chat is ready to use
    #[serde(rename(serialize = "secretChatStateReady", deserialize = "secretChatStateReady"))]
    Ready,
    /// The secret chat is closed
    #[serde(rename(serialize = "secretChatStateClosed", deserialize = "secretChatStateClosed"))]
    Closed,
}
