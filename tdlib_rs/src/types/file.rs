#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a file
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct File {
    /// Unique file identifier
    pub id: i32,
    /// File size, in bytes; 0 if unknown
    pub size: i64,
    /// Approximate file size in bytes in case the exact file size is unknown. Can be used to show download/upload progress
    pub expected_size: i64,
    /// Information about the local copy of the file
    pub local: crate::types::LocalFile,
    /// Information about the remote copy of the file
    pub remote: crate::types::RemoteFile,
}
