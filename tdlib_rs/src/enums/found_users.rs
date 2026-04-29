#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundUsers {
    /// Represents a list of found users
    #[serde(rename(serialize = "foundUsers", deserialize = "foundUsers"))]
    FoundUsers(crate::types::FoundUsers),
}
