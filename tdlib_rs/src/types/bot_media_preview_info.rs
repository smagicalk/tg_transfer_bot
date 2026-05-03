#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotMediaPreviewInfo {
    /// List of media previews
    pub previews: Vec<crate::types::BotMediaPreview>,
    /// List of language codes for which the bot has dedicated previews
    pub language_codes: Vec<String>,
}
