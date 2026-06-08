#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceFrontSide {
    /// Current hash of the file containing the front side
    pub file_hash: String,
}
