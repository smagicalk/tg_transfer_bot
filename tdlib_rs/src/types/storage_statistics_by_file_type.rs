#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains the storage usage statistics for a specific file type
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StorageStatisticsByFileType {
    /// File type
    pub file_type: crate::enums::FileType,
    /// Total size of the files, in bytes
    pub size: i64,
    /// Total number of files
    pub count: i32,
}
