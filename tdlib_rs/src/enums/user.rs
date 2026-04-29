#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum User {
    /// Represents a user
    #[serde(rename(serialize = "user", deserialize = "user"))]
    User(crate::types::User),
}
