#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes background of a gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftBackground {
    /// Center color in RGB format
    pub center_color: i32,
    /// Edge color in RGB format
    pub edge_color: i32,
    /// Text color in RGB format
    pub text_color: i32,
}
