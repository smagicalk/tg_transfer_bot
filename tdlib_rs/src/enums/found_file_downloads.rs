#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundFileDownloads {
    /// Contains a list of downloaded files, found by a search
    #[serde(rename(serialize = "foundFileDownloads", deserialize = "foundFileDownloads"))]
    FoundFileDownloads(crate::types::FoundFileDownloads),
}
