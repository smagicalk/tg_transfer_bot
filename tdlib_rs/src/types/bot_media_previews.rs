#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of media previews of a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreviews {
    /// List of media previews
    pub previews: Vec<crate::types::BotMediaPreview>,
}
