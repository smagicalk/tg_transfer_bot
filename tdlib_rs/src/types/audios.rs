#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of audio files
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Audios {
    /// Approximate total number of audio files found
    pub total_count: i32,
    /// List of audio files
    pub audios: Vec<crate::types::Audio>,
}
