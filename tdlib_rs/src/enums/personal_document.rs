#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PersonalDocument {
    /// A personal document, containing some information about a user
    #[serde(rename(serialize = "personalDocument", deserialize = "personalDocument"))]
    PersonalDocument(crate::types::PersonalDocument),
}
