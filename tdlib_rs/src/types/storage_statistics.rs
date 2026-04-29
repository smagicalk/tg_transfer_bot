#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains the exact storage usage statistics split by chats and file type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatistics {
    /// Total size of files, in bytes
    pub size: i64,
    /// Total number of files
    pub count: i32,
    /// Statistics split by chats
    pub by_chat: Vec<crate::types::StorageStatisticsByChat>,
}
