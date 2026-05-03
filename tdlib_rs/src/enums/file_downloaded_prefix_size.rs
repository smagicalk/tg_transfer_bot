#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FileDownloadedPrefixSize {
    /// Contains size of downloaded prefix of a file
    #[serde(rename(
        serialize = "fileDownloadedPrefixSize",
        deserialize = "fileDownloadedPrefixSize"
    ))]
    FileDownloadedPrefixSize(crate::types::FileDownloadedPrefixSize),
}
