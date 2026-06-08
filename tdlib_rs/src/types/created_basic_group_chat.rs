#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a newly created basic group chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreatedBasicGroupChat {
    /// Chat identifier
    pub chat_id: i64,
    /// Information about failed to add members
    pub failed_to_add_members: crate::types::FailedToAddMembers,
}
