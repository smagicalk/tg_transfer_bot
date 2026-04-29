#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents one language pack string
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackString {
    /// String key
    pub key: String,
    /// String value; pass null if the string needs to be taken from the built-in English language pack
    pub value: Option<crate::enums::LanguagePackStringValue>,
}
