#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of chat members
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMembers {
    /// Approximate total number of chat members found
    pub total_count: i32,
    /// A list of chat members
    pub members: Vec<crate::types::ChatMember>,
}
