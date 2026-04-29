#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a checklist
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChecklist {
    /// Checklist title
    pub title: String,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}
