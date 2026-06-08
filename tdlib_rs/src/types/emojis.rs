#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of emojis
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Emojis {
    /// List of emojis
    pub emojis: Vec<String>,
}
