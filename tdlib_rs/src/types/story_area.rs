#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a clickable rectangle area on a story media
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: crate::types::StoryAreaPosition,
    /// Type of the area
    pub r#type: crate::enums::StoryAreaType,
}
