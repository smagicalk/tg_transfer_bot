#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatPhotos {
    /// Contains a list of chat or user profile photos
    #[serde(rename(serialize = "chatPhotos", deserialize = "chatPhotos"))]
    ChatPhotos(crate::types::ChatPhotos),
}
