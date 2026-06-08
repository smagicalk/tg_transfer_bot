#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotoInfo {
    /// Contains basic information about the photo of a chat
    #[serde(rename(serialize = "chatPhotoInfo", deserialize = "chatPhotoInfo"))]
    ChatPhotoInfo(crate::types::ChatPhotoInfo),
}
