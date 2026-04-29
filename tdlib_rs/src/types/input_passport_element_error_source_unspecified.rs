#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceUnspecified {
    /// Current hash of the entire element
    pub element_hash: String,
}
