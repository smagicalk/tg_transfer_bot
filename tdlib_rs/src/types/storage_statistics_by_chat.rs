#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains the storage usage statistics for a specific chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatisticsByChat {
    /// Chat identifier; 0 if none
    pub chat_id: i64,
    /// Total size of the files in the chat, in bytes
    pub size: i64,
    /// Total number of files in the chat
    pub count: i32,
    /// Statistics split by file types
    pub by_file_type: Vec<crate::types::StorageStatisticsByFileType>,
}
