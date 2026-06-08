#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A supergroup has been created from a basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatUpgradeFrom {
    /// Title of the newly created supergroup
    pub title: String,
    /// The identifier of the original basic group
    pub basic_group_id: i64,
}
