#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat editable username was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventUsernameChanged {
    /// Previous chat username
    pub old_username: String,
    /// New chat username
    pub new_username: String,
}
