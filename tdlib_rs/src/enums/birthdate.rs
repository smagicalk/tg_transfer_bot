#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Birthdate {
    /// Represents a birthdate of a user
    #[serde(rename(serialize = "birthdate", deserialize = "birthdate"))]
    Birthdate(crate::types::Birthdate),
}
