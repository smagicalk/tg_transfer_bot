#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The selfie contains an error. The error is considered resolved when the file with the selfie changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceSelfie {
    /// Current hash of the file containing the selfie
    pub file_hash: String,
}
