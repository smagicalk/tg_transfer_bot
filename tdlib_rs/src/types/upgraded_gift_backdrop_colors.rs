#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes colors of a backdrop of an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftBackdropColors {
    /// A color in the center of the backdrop in the RGB format
    pub center_color: i32,
    /// A color on the edges of the backdrop in the RGB format
    pub edge_color: i32,
    /// A color to be applied for the symbol in the RGB format
    pub symbol_color: i32,
    /// A color for the text on the backdrop in the RGB format
    pub text_color: i32,
}
