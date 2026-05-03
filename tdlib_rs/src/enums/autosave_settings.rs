#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettings {
    /// Describes autosave settings
    #[serde(rename(serialize = "autosaveSettings", deserialize = "autosaveSettings"))]
    AutosaveSettings(crate::types::AutosaveSettings),
}
