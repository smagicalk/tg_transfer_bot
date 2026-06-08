#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutoDownloadSettings {
    /// Contains auto-download settings
    #[serde(rename(
        serialize = "autoDownloadSettings",
        deserialize = "autoDownloadSettings"
    ))]
    AutoDownloadSettings(crate::types::AutoDownloadSettings),
}
