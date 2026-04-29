#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes usernames assigned to a user, a supergroup, or a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Usernames {
    /// List of active usernames; the first one must be shown as the primary username. The order of active usernames can be changed with reorderActiveUsernames, reorderBotActiveUsernames or reorderSupergroupActiveUsernames
    pub active_usernames: Vec<String>,
    /// List of currently disabled usernames; the username can be activated with toggleUsernameIsActive, toggleBotUsernameIsActive, or toggleSupergroupUsernameIsActive
    pub disabled_usernames: Vec<String>,
    /// Active or disabled username, which may be changed with setUsername or setSupergroupUsername
    pub editable_username: String,
    /// Collectible usernames that were purchased at https:fragment.com and can be passed to getCollectibleItemInfo for more details
    pub collectible_usernames: Vec<String>,
}
