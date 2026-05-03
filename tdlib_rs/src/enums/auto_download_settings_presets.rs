#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutoDownloadSettingsPresets {
    /// Contains auto-download settings presets for the current user
    #[serde(rename(
        serialize = "autoDownloadSettingsPresets",
        deserialize = "autoDownloadSettingsPresets"
    ))]
    AutoDownloadSettingsPresets(crate::types::AutoDownloadSettingsPresets),
}
