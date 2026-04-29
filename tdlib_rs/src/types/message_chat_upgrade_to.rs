#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A basic group was upgraded to a supergroup and was deactivated as the result
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatUpgradeTo {
    /// Identifier of the supergroup to which the basic group was upgraded
    pub supergroup_id: i64,
}
