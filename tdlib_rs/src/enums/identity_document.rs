#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum IdentityDocument {
    /// An identity document
    #[serde(rename(serialize = "identityDocument", deserialize = "identityDocument"))]
    IdentityDocument(crate::types::IdentityDocument),
}
