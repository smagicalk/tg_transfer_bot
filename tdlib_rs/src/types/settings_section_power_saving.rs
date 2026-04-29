#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The power saving settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPowerSaving {
    /// Subsection of the section; may be one of
    /// "", "videos", "gifs", "stickers", "emoji", "effects", "preload", "background", "call-animations", "particles", "transitions"
    pub subsection: String,
}
