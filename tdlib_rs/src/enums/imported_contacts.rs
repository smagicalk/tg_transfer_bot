#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ImportedContacts {
    /// Represents the result of an importContacts request
    #[serde(rename(serialize = "importedContacts", deserialize = "importedContacts"))]
    ImportedContacts(crate::types::ImportedContacts),
}
