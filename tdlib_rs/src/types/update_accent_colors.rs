#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of supported accent colors has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAccentColors {
    /// Information about supported colors; colors with identifiers 0 (red), 1 (orange), 2 (purple/violet), 3 (green), 4 (cyan), 5 (blue), 6 (pink) must always be supported
    /// and aren't included in the list. The exact colors for the accent colors with identifiers 0-6 must be taken from the app theme
    pub colors: Vec<crate::types::AccentColor>,
    /// The list of accent color identifiers, which can be set through setAccentColor and setChatAccentColor. The colors must be shown in the specified order
    pub available_accent_color_ids: Vec<i32>,
}
