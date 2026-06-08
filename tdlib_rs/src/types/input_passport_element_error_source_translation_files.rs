#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The translation of the document contains an error. The error is considered resolved when the list of files changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPassportElementErrorSourceTranslationFiles {
    /// Current hashes of all files with the translation
    pub file_hashes: Vec<String>,
}
