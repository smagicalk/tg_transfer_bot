#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of chat themes based on upgraded gifts
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftChatThemes {
    /// A list of chat themes
    pub themes: Vec<crate::types::GiftChatTheme>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
