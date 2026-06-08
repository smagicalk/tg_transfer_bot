#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The log is written to a file
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogStreamFile {
    /// Path to the file to where the internal TDLib log will be written
    pub path: String,
    /// The maximum size of the file to where the internal TDLib log is written before the file will automatically be rotated, in bytes
    pub max_file_size: i64,
    /// Pass true to additionally redirect stderr to the log file. Ignored on Windows
    pub redirect_stderr: bool,
}
