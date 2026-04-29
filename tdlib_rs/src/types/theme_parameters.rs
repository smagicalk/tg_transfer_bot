#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains parameters of the application theme
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ThemeParameters {
    /// A color of the background in the RGB format
    pub background_color: i32,
    /// A secondary color for the background in the RGB format
    pub secondary_background_color: i32,
    /// A color of the header background in the RGB format
    pub header_background_color: i32,
    /// A color of the bottom bar background in the RGB format
    pub bottom_bar_background_color: i32,
    /// A color of the section background in the RGB format
    pub section_background_color: i32,
    /// A color of the section separator in the RGB format
    pub section_separator_color: i32,
    /// A color of text in the RGB format
    pub text_color: i32,
    /// An accent color of the text in the RGB format
    pub accent_text_color: i32,
    /// A color of text on the section headers in the RGB format
    pub section_header_text_color: i32,
    /// A color of the subtitle text in the RGB format
    pub subtitle_text_color: i32,
    /// A color of the text for destructive actions in the RGB format
    pub destructive_text_color: i32,
    /// A color of hints in the RGB format
    pub hint_color: i32,
    /// A color of links in the RGB format
    pub link_color: i32,
    /// A color of the buttons in the RGB format
    pub button_color: i32,
    /// A color of text on the buttons in the RGB format
    pub button_text_color: i32,
}
