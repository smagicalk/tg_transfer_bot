#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ArchiveChatListSettings {
    /// Contains settings for automatic moving of chats to and from the Archive chat lists
    #[serde(rename(serialize = "archiveChatListSettings", deserialize = "archiveChatListSettings"))]
    ArchiveChatListSettings(crate::types::ArchiveChatListSettings),
}
