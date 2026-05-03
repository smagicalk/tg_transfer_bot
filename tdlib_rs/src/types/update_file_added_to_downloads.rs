#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateFileAddedToDownloads {
    /// The added file download
    pub file_download: crate::types::FileDownload,
    /// New number of being downloaded and recently downloaded files found
    pub counts: crate::types::DownloadedFileCounts,
}
