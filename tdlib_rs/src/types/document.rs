#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a document of any type
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Document {
    /// Original name of the file; as defined by the sender
    pub file_name: String,
    /// MIME type of the file; as defined by the sender
    pub mime_type: String,
    /// Document minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Document thumbnail in JPEG or PNG format (PNG will be used only for background patterns); as defined by the sender; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// File containing the document
    pub document: crate::types::File,
}
