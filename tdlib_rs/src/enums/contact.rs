#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Contact {
    /// Describes a contact of a user
    #[serde(rename(serialize = "contact", deserialize = "contact"))]
    Contact(crate::types::Contact),
}
