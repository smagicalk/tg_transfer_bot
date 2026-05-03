#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BasicGroup {
    /// Group identifier
    pub id: i64,
    /// Number of members in the group
    pub member_count: i32,
    /// Status of the current user in the group
    pub status: crate::enums::ChatMemberStatus,
    /// True, if the group is active
    pub is_active: bool,
    /// Identifier of the supergroup to which this group was upgraded; 0 if none
    pub upgraded_to_supergroup_id: i64,
}
