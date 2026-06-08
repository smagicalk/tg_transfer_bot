#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatistics {
    /// Contains the exact storage usage statistics split by chats and file type
    #[serde(rename(serialize = "storageStatistics", deserialize = "storageStatistics"))]
    StorageStatistics(crate::types::StorageStatistics),
}
