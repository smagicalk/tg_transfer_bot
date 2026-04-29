#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data in supergroupFullInfo has been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSupergroupFullInfo {
    /// Identifier of the supergroup or channel
    pub supergroup_id: i64,
    /// New full information about the supergroup
    pub supergroup_full_info: crate::types::SupergroupFullInfo,
}
