#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains approximate storage usage statistics, excluding files of unknown file type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StorageStatisticsFast {
    /// Approximate total size of files, in bytes
    pub files_size: i64,
    /// Approximate number of files
    pub file_count: i32,
    /// Size of the database
    pub database_size: i64,
    /// Size of the language pack database
    pub language_pack_database_size: i64,
    /// Size of the TDLib internal log
    pub log_size: i64,
}
