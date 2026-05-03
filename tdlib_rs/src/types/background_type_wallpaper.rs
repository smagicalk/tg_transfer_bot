#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A wallpaper in JPEG format
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeWallpaper {
    /// True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12
    pub is_blurred: bool,
    /// True, if the background needs to be slightly moved when device is tilted
    pub is_moving: bool,
}
