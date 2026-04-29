#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceReverseSide {
    /// Current hash of the file containing the reverse side
    pub file_hash: String,
}
