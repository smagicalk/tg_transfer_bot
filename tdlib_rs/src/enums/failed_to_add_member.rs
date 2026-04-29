#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FailedToAddMember {
    /// Contains information about a user who has failed to be added to a chat
    #[serde(rename(serialize = "failedToAddMember", deserialize = "failedToAddMember"))]
    FailedToAddMember(crate::types::FailedToAddMember),
}
