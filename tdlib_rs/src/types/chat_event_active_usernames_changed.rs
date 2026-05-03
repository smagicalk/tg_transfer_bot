#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat active usernames were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventActiveUsernamesChanged {
    /// Previous list of active usernames
    pub old_usernames: Vec<String>,
    /// New list of active usernames
    pub new_usernames: Vec<String>,
}
