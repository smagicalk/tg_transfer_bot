#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An audio file
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockAudio {
    /// Audio file; may be null
    pub audio: Option<crate::types::Audio>,
    /// Audio file caption
    pub caption: crate::types::PageBlockCaption,
}
