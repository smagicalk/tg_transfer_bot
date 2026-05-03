#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorSourceTranslationFile {
    /// Index of a file with the error
    pub file_index: i32,
}
