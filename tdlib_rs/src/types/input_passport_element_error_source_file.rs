#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The file contains an error. The error is considered resolved when the file changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFile {
    /// Current hash of the file which has the error
    pub file_hash: String,
}
