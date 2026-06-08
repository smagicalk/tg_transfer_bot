#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of available TDLib internal log tags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LogTags {
    /// List of log tags
    pub tags: Vec<String>,
}
