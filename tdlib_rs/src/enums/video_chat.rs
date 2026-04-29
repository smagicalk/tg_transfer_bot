#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoChat {
    /// Describes a video chat, i.e. a group call bound to a chat
    #[serde(rename(serialize = "videoChat", deserialize = "videoChat"))]
    VideoChat(crate::types::VideoChat),
}
