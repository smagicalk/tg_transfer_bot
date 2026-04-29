#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A file download was changed. This update is sent only after file download list is loaded for the first time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFileDownload {
    /// File identifier
    pub file_id: i32,
    /// Point in time (Unix timestamp) when the file downloading was completed; 0 if the file downloading isn't completed
    pub complete_date: i32,
    /// True, if downloading of the file is paused
    pub is_paused: bool,
    /// New number of being downloaded and recently downloaded files found
    pub counts: crate::types::DownloadedFileCounts,
}
