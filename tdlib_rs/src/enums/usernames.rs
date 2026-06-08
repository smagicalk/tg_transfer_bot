#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Usernames {
    /// Describes usernames assigned to a user, a supergroup, or a channel
    #[serde(rename(serialize = "usernames", deserialize = "usernames"))]
    Usernames(crate::types::Usernames),
}
