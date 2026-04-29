#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatisticsByFileType {
    /// Contains the storage usage statistics for a specific file type
    #[serde(rename(serialize = "storageStatisticsByFileType", deserialize = "storageStatisticsByFileType"))]
    StorageStatisticsByFileType(crate::types::StorageStatisticsByFileType),
}
