#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains auto-download settings presets for the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoDownloadSettingsPresets {
    /// Preset with lowest settings; expected to be used by default when roaming
    pub low: crate::types::AutoDownloadSettings,
    /// Preset with medium settings; expected to be used by default when using mobile data
    pub medium: crate::types::AutoDownloadSettings,
    /// Preset with highest settings; expected to be used by default when connected on Wi-Fi
    pub high: crate::types::AutoDownloadSettings,
}
