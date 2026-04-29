#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of language pack strings
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStrings {
    /// A list of language pack strings
    pub strings: Vec<crate::types::LanguagePackString>,
}
