#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains size of downloaded prefix of a file
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FileDownloadedPrefixSize {
    /// The prefix size, in bytes
    pub size: i64,
}
