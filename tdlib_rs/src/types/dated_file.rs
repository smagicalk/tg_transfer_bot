#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// File with the date it was uploaded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DatedFile {
    /// The file
    pub file: crate::types::File,
    /// Point in time (Unix timestamp) when the file was uploaded
    pub date: i32,
}
