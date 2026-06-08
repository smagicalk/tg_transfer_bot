#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An anchor
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RichTextAnchor {
    /// Anchor name
    pub name: String,
}
