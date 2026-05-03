#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SpeechRecognitionResult {
    /// The speech recognition is ongoing
    #[serde(rename(
        serialize = "speechRecognitionResultPending",
        deserialize = "speechRecognitionResultPending"
    ))]
    Pending(crate::types::SpeechRecognitionResultPending),
    /// The speech recognition successfully finished
    #[serde(rename(
        serialize = "speechRecognitionResultText",
        deserialize = "speechRecognitionResultText"
    ))]
    Text(crate::types::SpeechRecognitionResultText),
    /// The speech recognition failed
    #[serde(rename(
        serialize = "speechRecognitionResultError",
        deserialize = "speechRecognitionResultError"
    ))]
    Error(crate::types::SpeechRecognitionResultError),
}
