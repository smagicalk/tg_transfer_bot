#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The speech recognition is ongoing
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultPending {
    /// Partially recognized text
    pub partial_text: String,
}
