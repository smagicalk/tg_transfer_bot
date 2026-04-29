#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains number of being downloaded and recently downloaded files found
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DownloadedFileCounts {
    /// Number of active file downloads found, including paused
    pub active_count: i32,
    /// Number of paused file downloads found
    pub paused_count: i32,
    /// Number of completed file downloads found
    pub completed_count: i32,
}
