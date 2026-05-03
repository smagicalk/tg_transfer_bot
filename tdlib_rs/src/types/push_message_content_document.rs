#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A document message (a general file)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentDocument {
    /// Message content; may be null
    pub document: Option<crate::types::Document>,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
