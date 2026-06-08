#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderIcon {
    /// Represents an icon for a chat folder
    #[serde(rename(serialize = "chatFolderIcon", deserialize = "chatFolderIcon"))]
    ChatFolderIcon(crate::types::ChatFolderIcon),
}
