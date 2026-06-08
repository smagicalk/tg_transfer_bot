#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to application settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeSettings {
    /// Section of the application settings to open; may be null if none
    pub section: Option<crate::enums::SettingsSection>,
}
