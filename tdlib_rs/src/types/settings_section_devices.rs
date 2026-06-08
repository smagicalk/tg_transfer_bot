#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The Devices section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionDevices {
    /// Subsection of the section; may be one of
    /// "", "edit", "link-desktop", "terminate-sessions", "auto-terminate"
    pub subsection: String,
}
