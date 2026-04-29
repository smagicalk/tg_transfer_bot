#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The parameters of speech recognition without Telegram Premium subscription has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateSpeechRecognitionTrial {
    /// The maximum allowed duration of media for speech recognition without Telegram Premium subscription, in seconds
    pub max_media_duration: i32,
    /// The total number of allowed speech recognitions per week; 0 if none
    pub weekly_count: i32,
    /// Number of left speech recognition attempts this week
    pub left_count: i32,
    /// Point in time (Unix timestamp) when the weekly number of tries will reset; 0 if unknown
    pub next_reset_date: i32,
}
