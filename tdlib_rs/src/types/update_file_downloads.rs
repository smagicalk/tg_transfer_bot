#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The state of the file download list has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFileDownloads {
    /// Total size of files in the file download list, in bytes
    pub total_size: i64,
    /// Total number of files in the file download list
    pub total_count: i32,
    /// Total downloaded size of files in the file download list, in bytes
    pub downloaded_size: i64,
}
