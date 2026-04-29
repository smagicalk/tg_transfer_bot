#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettingsException {
    /// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
    #[serde(rename(serialize = "autosaveSettingsException", deserialize = "autosaveSettingsException"))]
    AutosaveSettingsException(crate::types::AutosaveSettingsException),
}
