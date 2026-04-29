#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypePattern {
    /// Fill of the background
    pub fill: crate::enums::BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: i32,
    /// True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    pub is_inverted: bool,
    /// True, if the background needs to be slightly moved when device is tilted
    pub is_moving: bool,
}
