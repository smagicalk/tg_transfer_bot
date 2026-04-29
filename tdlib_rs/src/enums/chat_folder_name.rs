#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderName {
    /// Describes name of a chat folder
    #[serde(rename(serialize = "chatFolderName", deserialize = "chatFolderName"))]
    ChatFolderName(crate::types::ChatFolderName),
}
