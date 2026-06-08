#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ImportedContact {
    /// Describes a contact to import
    #[serde(rename(serialize = "importedContact", deserialize = "importedContact"))]
    ImportedContact(crate::types::ImportedContact),
}
