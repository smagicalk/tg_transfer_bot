#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInfo {
    /// Contains basic information about a chat folder
    #[serde(rename(serialize = "chatFolderInfo", deserialize = "chatFolderInfo"))]
    ChatFolderInfo(crate::types::ChatFolderInfo),
}
