#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhoto {
    /// Describes a chat or user profile photo
    #[serde(rename(serialize = "chatPhoto", deserialize = "chatPhoto"))]
    ChatPhoto(crate::types::ChatPhoto),
}
