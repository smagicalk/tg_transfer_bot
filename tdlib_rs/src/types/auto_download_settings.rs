#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains auto-download settings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutoDownloadSettings {
    /// True, if the auto-download is enabled
    pub is_auto_download_enabled: bool,
    /// The maximum size of a photo file to be auto-downloaded, in bytes
    pub max_photo_file_size: i32,
    /// The maximum size of a video file to be auto-downloaded, in bytes
    pub max_video_file_size: i64,
    /// The maximum size of other file types to be auto-downloaded, in bytes
    pub max_other_file_size: i64,
    /// The maximum suggested bitrate for uploaded videos, in kbit/s
    pub video_upload_bitrate: i32,
    /// True, if the beginning of video files needs to be preloaded for instant playback
    pub preload_large_videos: bool,
    /// True, if the next audio track needs to be preloaded while the user is listening to an audio file
    pub preload_next_audio: bool,
    /// True, if stories needs to be preloaded
    pub preload_stories: bool,
    /// True, if "use less data for calls" option needs to be enabled
    pub use_less_data_for_calls: bool,
}
