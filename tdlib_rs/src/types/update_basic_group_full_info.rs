#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data in basicGroupFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateBasicGroupFullInfo {
    /// Identifier of a basic group
    pub basic_group_id: i64,
    /// New full information about the group
    pub basic_group_full_info: crate::types::BasicGroupFullInfo,
}
