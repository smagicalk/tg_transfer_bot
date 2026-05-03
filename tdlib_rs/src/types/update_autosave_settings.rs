#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Autosave settings for some type of chats were updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAutosaveSettings {
    /// Type of chats for which autosave settings were updated
    pub scope: crate::enums::AutosaveSettingsScope,
    /// The new autosave settings; may be null if the settings are reset to default
    pub settings: Option<crate::types::ScopeAutosaveSettings>,
}
