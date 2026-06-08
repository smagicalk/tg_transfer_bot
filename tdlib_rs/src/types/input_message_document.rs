#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A document message (general file)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputMessageDocument {
    /// Document to be sent
    pub document: crate::enums::InputFile,
    /// Document thumbnail; pass null to skip thumbnail uploading
    pub thumbnail: Option<crate::types::InputThumbnail>,
    /// Pass true to disable automatic file type detection and send the document as a file. Always true for files sent to secret chats
    pub disable_content_type_detection: bool,
    /// Document caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
    pub caption: Option<crate::types::FormattedText>,
}
