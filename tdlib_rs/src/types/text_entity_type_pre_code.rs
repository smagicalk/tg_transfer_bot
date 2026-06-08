#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Text that must be formatted as if inside pre, and code HTML tags
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextEntityTypePreCode {
    /// Programming language of the code; as defined by the sender
    pub language: String,
}
