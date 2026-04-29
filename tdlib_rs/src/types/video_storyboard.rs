#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a storyboard for a video
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VideoStoryboard {
    /// A JPEG file that contains tiled previews of video
    pub storyboard_file: crate::types::File,
    /// Width of a tile
    pub width: i32,
    /// Height of a tile
    pub height: i32,
    /// File that describes mapping of position in the video to a tile in the JPEG file
    pub map_file: crate::types::File,
}
