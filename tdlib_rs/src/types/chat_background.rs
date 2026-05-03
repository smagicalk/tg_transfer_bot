#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a background set for a specific chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatBackground {
    /// The background
    pub background: crate::types::Background,
    /// Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
    pub dark_theme_dimming: i32,
}
