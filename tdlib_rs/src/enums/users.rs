#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Users {
    /// Represents a list of users
    #[serde(rename(serialize = "users", deserialize = "users"))]
    Users(crate::types::Users),
}
