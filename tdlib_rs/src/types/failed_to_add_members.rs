#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of users that has failed to be added to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FailedToAddMembers {
    /// Information about users that weren't added to the chat
    pub failed_to_add_members: Vec<crate::types::FailedToAddMember>,
}
