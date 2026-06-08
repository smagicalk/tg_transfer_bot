#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An audio message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageAudio {
    /// The audio description
    pub audio: crate::types::Audio,
    /// Audio caption
    pub caption: crate::types::FormattedText,
}
