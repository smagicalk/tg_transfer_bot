#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A newly created basic group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageBasicGroupChatCreate {
    /// Title of the basic group
    pub title: String,
    /// User identifiers of members in the basic group
    pub member_user_ids: Vec<i64>,
}
