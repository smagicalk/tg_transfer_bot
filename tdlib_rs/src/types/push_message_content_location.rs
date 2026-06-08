#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A message with a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentLocation {
    /// True, if the location is live
    pub is_live: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
