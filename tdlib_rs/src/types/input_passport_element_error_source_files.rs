#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of attached files contains an error. The error is considered resolved when the file list changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFiles {
    /// Current hashes of all attached files
    pub file_hashes: Vec<String>,
}
