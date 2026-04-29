#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackInfo {
    /// Contains information about a language pack
    #[serde(rename(serialize = "languagePackInfo", deserialize = "languagePackInfo"))]
    LanguagePackInfo(crate::types::LanguagePackInfo),
}
