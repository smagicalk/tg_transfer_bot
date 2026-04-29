#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about supported accent colors for user profile photo background in RGB format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProfileAccentColors {
    /// The list of 1-2 colors in RGB format, describing the colors, as expected to be shown in the color palette settings
    pub palette_colors: Vec<i32>,
    /// The list of 1-2 colors in RGB format, describing the colors, as expected to be used for the profile photo background
    pub background_colors: Vec<i32>,
    /// The list of 2 colors in RGB format, describing the colors of the gradient to be used for the unread active story indicator around profile photo
    pub story_colors: Vec<i32>,
}
