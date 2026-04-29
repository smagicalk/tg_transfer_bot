#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FileDownload {
    /// Describes a file added to file download list
    #[serde(rename(serialize = "fileDownload", deserialize = "fileDownload"))]
    FileDownload(crate::types::FileDownload),
}
