#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A quick reply shortcut and all its messages were deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcutDeleted {
    /// The identifier of the deleted shortcut
    pub shortcut_id: i32,
}
