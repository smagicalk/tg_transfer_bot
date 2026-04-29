#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DownloadedFileCounts {
    /// Contains number of being downloaded and recently downloaded files found
    #[serde(rename(serialize = "downloadedFileCounts", deserialize = "downloadedFileCounts"))]
    DownloadedFileCounts(crate::types::DownloadedFileCounts),
}
