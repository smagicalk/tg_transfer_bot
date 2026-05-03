#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The media is hidden until the invoice is paid
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaidMediaPreview {
    /// Media width; 0 if unknown
    pub width: i32,
    /// Media height; 0 if unknown
    pub height: i32,
    /// Media duration, in seconds; 0 if unknown
    pub duration: i32,
    /// Media minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
}
