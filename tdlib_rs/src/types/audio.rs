#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an audio file. Audio is usually in MP3 or M4A format
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Audio {
    /// Duration of the audio, in seconds; as defined by the sender
    pub duration: i32,
    /// Title of the audio; as defined by the sender
    pub title: String,
    /// Performer of the audio; as defined by the sender
    pub performer: String,
    /// Original name of the file; as defined by the sender
    pub file_name: String,
    /// The MIME type of the file; as defined by the sender
    pub mime_type: String,
    /// The minithumbnail of the album cover; may be null
    pub album_cover_minithumbnail: Option<crate::types::Minithumbnail>,
    /// The thumbnail of the album cover in JPEG format; as defined by the sender. The full size thumbnail is expected to be extracted from the downloaded audio file; may be null
    pub album_cover_thumbnail: Option<crate::types::Thumbnail>,
    /// Album cover variants to use if the downloaded audio file contains no album cover. Provided thumbnail dimensions are approximate
    pub external_album_covers: Vec<crate::types::Thumbnail>,
    /// File containing the audio
    pub audio: crate::types::File,
}
