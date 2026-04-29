#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An audio message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageAudio {
    /// Audio file to be sent
    pub audio: crate::enums::InputFile,
    /// Thumbnail of the cover for the album; pass null to skip thumbnail uploading
    pub album_cover_thumbnail: Option<crate::types::InputThumbnail>,
    /// Duration of the audio, in seconds; may be replaced by the server
    pub duration: i32,
    /// Title of the audio; 0-64 characters; may be replaced by the server
    pub title: String,
    /// Performer of the audio; 0-64 characters, may be replaced by the server
    pub performer: String,
    /// Audio caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
}
