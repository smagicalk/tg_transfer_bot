#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a file added to file download list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FileDownload {
    /// File identifier
    pub file_id: i32,
    /// The message with the file
    pub message: crate::types::Message,
    /// Point in time (Unix timestamp) when the file was added to the download list
    pub add_date: i32,
    /// Point in time (Unix timestamp) when the file downloading was completed; 0 if the file downloading isn't completed
    pub complete_date: i32,
    /// True, if downloading of the file is paused
    pub is_paused: bool,
}
