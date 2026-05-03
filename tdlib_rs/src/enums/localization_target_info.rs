#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LocalizationTargetInfo {
    /// Contains information about the current localization target
    #[serde(rename(
        serialize = "localizationTargetInfo",
        deserialize = "localizationTargetInfo"
    ))]
    LocalizationTargetInfo(crate::types::LocalizationTargetInfo),
}
