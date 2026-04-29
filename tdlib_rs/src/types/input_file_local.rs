#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A file defined by a local path
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputFileLocal {
    /// Local path to the file
    pub path: String,
}
