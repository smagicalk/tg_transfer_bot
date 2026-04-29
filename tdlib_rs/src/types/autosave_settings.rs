#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes autosave settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettings {
    /// Default autosave settings for private chats
    pub private_chat_settings: crate::types::ScopeAutosaveSettings,
    /// Default autosave settings for basic group and supergroup chats
    pub group_settings: crate::types::ScopeAutosaveSettings,
    /// Default autosave settings for channel chats
    pub channel_settings: crate::types::ScopeAutosaveSettings,
    /// Autosave settings for specific chats
    pub exceptions: Vec<crate::types::AutosaveSettingsException>,
}
