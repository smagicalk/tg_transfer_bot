#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RemoteFile {
    /// Represents a remote file
    #[serde(rename(serialize = "remoteFile", deserialize = "remoteFile"))]
    RemoteFile(crate::types::RemoteFile),
}
