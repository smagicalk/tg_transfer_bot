#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatFolderInviteLinks {
    /// Represents a list of chat folder invite links
    #[serde(rename(
        serialize = "chatFolderInviteLinks",
        deserialize = "chatFolderInviteLinks"
    ))]
    ChatFolderInviteLinks(crate::types::ChatFolderInviteLinks),
}
