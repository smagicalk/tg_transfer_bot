#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsException {
    /// Chat identifier
    pub chat_id: i64,
    /// Autosave settings for the chat
    pub settings: crate::types::ScopeAutosaveSettings,
}
