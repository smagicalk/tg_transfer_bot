#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of supported accent colors for user profiles has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateProfileAccentColors {
    /// Information about supported colors
    pub colors: Vec<crate::types::ProfileAccentColor>,
    /// The list of accent color identifiers, which can be set through setProfileAccentColor and setChatProfileAccentColor. The colors must be shown in the specified order
    pub available_accent_color_ids: Vec<i32>,
}
