#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A text message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentText {
    /// Message text
    pub text: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
