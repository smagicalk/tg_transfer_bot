#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of emoji categories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmojiCategories {
    /// List of categories
    pub categories: Vec<crate::types::EmojiCategory>,
}
