#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateFileRemovedFromDownloads {
    /// File identifier
    pub file_id: i32,
    /// New number of being downloaded and recently downloaded files found
    pub counts: crate::types::DownloadedFileCounts,
}
