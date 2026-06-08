#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat title was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventTitleChanged {
    /// Previous chat title
    pub old_title: String,
    /// New chat title
    pub new_title: String,
}
