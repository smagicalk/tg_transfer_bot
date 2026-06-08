#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatisticsByChat {
    /// Contains the storage usage statistics for a specific chat
    #[serde(rename(
        serialize = "storageStatisticsByChat",
        deserialize = "storageStatisticsByChat"
    ))]
    StorageStatisticsByChat(crate::types::StorageStatisticsByChat),
}
