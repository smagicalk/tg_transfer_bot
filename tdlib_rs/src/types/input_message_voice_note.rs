#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A voice note message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageVoiceNote {
    /// Voice note to be sent. The voice note must be encoded with the Opus codec and stored inside an OGG container with a single audio channel, or be in MP3 or M4A format as regular audio
    pub voice_note: crate::enums::InputFile,
    /// Duration of the voice note, in seconds
    pub duration: i32,
    /// Waveform representation of the voice note in 5-bit format
    pub waveform: String,
    /// Voice note caption; may be null if empty; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
    /// Voice note self-destruct type; may be null if none; pass null if none; private chats only
    pub self_destruct_type: Option<crate::enums::MessageSelfDestructType>,
}
