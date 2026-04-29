#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of quick reply shortcuts has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateQuickReplyShortcuts {
    /// The new list of identifiers of quick reply shortcuts
    pub shortcut_ids: Vec<i32>,
}
