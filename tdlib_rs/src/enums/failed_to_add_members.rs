#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FailedToAddMembers {
    /// Represents a list of users that has failed to be added to a chat
    #[serde(rename(serialize = "failedToAddMembers", deserialize = "failedToAddMembers"))]
    FailedToAddMembers(crate::types::FailedToAddMembers),
}
