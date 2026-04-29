#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackString {
    /// Represents one language pack string
    #[serde(rename(serialize = "languagePackString", deserialize = "languagePackString"))]
    LanguagePackString(crate::types::LanguagePackString),
}
