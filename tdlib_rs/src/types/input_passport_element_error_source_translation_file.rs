#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceTranslationFile {
    /// Current hash of the file containing the translation
    pub file_hash: String,
}
