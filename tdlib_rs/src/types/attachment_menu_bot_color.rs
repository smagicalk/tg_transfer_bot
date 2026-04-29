#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a color to highlight a bot added to attachment menu
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AttachmentMenuBotColor {
    /// Color in the RGB format for light themes
    pub light_color: i32,
    /// Color in the RGB format for dark themes
    pub dark_color: i32,
}
