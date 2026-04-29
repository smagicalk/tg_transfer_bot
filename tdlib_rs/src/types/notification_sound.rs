#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a notification sound in MP3 format
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationSound {
    /// Unique identifier of the notification sound
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Duration of the sound, in seconds
    pub duration: i32,
    /// Point in time (Unix timestamp) when the sound was created
    pub date: i32,
    /// Title of the notification sound
    pub title: String,
    /// Arbitrary data, defined while the sound was uploaded
    pub data: String,
    /// File containing the sound
    pub sound: crate::types::File,
}
