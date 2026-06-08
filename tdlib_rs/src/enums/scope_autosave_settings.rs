#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ScopeAutosaveSettings {
    /// Contains autosave settings for an autosave settings scope
    #[serde(rename(
        serialize = "scopeAutosaveSettings",
        deserialize = "scopeAutosaveSettings"
    ))]
    ScopeAutosaveSettings(crate::types::ScopeAutosaveSettings),
}
