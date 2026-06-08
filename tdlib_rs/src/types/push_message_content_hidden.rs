#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A general message with hidden content
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentHidden {
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
