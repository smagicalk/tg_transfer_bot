#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an available stream in a video chat or a live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupCallStream {
    /// Identifier of an audio/video channel
    pub channel_id: i32,
    /// Scale of segment durations in the stream. The duration is 1000/(2**scale) milliseconds
    pub scale: i32,
    /// Point in time when the stream currently ends; Unix timestamp in milliseconds
    pub time_offset: i64,
}
