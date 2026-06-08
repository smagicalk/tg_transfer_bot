#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStrings {
    /// Contains a list of language pack strings
    #[serde(rename(serialize = "languagePackStrings", deserialize = "languagePackStrings"))]
    LanguagePackStrings(crate::types::LanguagePackStrings),
}
