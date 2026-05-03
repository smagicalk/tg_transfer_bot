#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatisticsFast {
    /// Contains approximate storage usage statistics, excluding files of unknown file type
    #[serde(rename(
        serialize = "storageStatisticsFast",
        deserialize = "storageStatisticsFast"
    ))]
    StorageStatisticsFast(crate::types::StorageStatisticsFast),
}
