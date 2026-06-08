#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The speech recognition failed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SpeechRecognitionResultError {
    /// Recognition error. An error with a message "MSG_VOICE_TOO_LONG" is returned when media duration is too big to be recognized
    pub error: crate::types::Error,
}
