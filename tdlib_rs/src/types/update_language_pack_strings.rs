#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Some language pack strings have been updated
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateLanguagePackStrings {
    /// Localization target to which the language pack belongs
    pub localization_target: String,
    /// Identifier of the updated language pack
    pub language_pack_id: String,
    /// List of changed language pack strings; empty if all strings have changed
    pub strings: Vec<crate::types::LanguagePackString>,
}
