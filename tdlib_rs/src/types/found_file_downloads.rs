#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of downloaded files, found by a search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundFileDownloads {
    /// Total number of suitable files, ignoring offset
    pub total_counts: crate::types::DownloadedFileCounts,
    /// The list of files
    pub files: Vec<crate::types::FileDownload>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
