#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LocalFile {
    /// Represents a local file
    #[serde(rename(serialize = "localFile", deserialize = "localFile"))]
    LocalFile(crate::types::LocalFile),
}
