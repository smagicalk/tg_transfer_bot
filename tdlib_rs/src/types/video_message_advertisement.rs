#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an advertisent to be shown while a video from a message is watched
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VideoMessageAdvertisement {
    /// Unique identifier of this result
    pub unique_id: i64,
    /// Text of the advertisement
    pub text: String,
    /// The minimum amount of time the advertisement must be displayed before it can be hidden by the user, in seconds
    pub min_display_duration: i32,
    /// The maximum amount of time the advertisement must be displayed before it must be automatically hidden, in seconds
    pub max_display_duration: i32,
    /// True, if the advertisement can be reported to Telegram moderators through reportVideoMessageAdvertisement
    pub can_be_reported: bool,
    /// Information about the sponsor of the advertisement
    pub sponsor: crate::types::AdvertisementSponsor,
    /// Title of the sponsored message
    pub title: String,
    /// If non-empty, additional information about the sponsored message to be shown along with the message
    pub additional_info: String,
}
