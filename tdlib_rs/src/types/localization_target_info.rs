#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about the current localization target
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LocalizationTargetInfo {
    /// List of available language packs for this application
    pub language_packs: Vec<crate::types::LanguagePackInfo>,
}
