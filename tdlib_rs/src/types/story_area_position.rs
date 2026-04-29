#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes position of a clickable rectangle area on a story media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the rectangle's center, as a percentage of the media width
    pub x_percentage: f64,
    /// The ordinate of the rectangle's center, as a percentage of the media height
    pub y_percentage: f64,
    /// The width of the rectangle, as a percentage of the media width
    pub width_percentage: f64,
    /// The height of the rectangle, as a percentage of the media height
    pub height_percentage: f64,
    /// Clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    pub corner_radius_percentage: f64,
}
