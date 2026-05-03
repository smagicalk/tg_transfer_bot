#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The speech recognition successfully finished
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultText {
    /// Recognized text
    pub text: String,
}
