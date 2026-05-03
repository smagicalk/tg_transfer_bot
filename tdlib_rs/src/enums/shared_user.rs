#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SharedUser {
    /// Contains information about a user shared with a bot
    #[serde(rename(serialize = "sharedUser", deserialize = "sharedUser"))]
    SharedUser(crate::types::SharedUser),
}
