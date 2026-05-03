#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains autosave settings for an autosave settings scope
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ScopeAutosaveSettings {
    /// True, if photo autosave is enabled
    pub autosave_photos: bool,
    /// True, if video autosave is enabled
    pub autosave_videos: bool,
    /// The maximum size of a video file to be autosaved, in bytes; 512 KB - 4000 MB
    pub max_video_file_size: i64,
}
