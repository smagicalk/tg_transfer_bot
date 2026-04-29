#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a language pack
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackInfo {
    /// Unique language pack identifier
    pub id: String,
    /// Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it must be fetched from base language pack. Unsupported in custom language packs
    pub base_language_pack_id: String,
    /// Language name
    pub name: String,
    /// Name of the language in that language
    pub native_name: String,
    /// A language code to be used to apply plural forms. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
    pub plural_code: String,
    /// True, if the language pack is official
    pub is_official: bool,
    /// True, if the language pack strings are RTL
    pub is_rtl: bool,
    /// True, if the language pack is a beta language pack
    pub is_beta: bool,
    /// True, if the language pack is installed by the current user
    pub is_installed: bool,
    /// Total number of non-deleted strings from the language pack
    pub total_string_count: i32,
    /// Total number of translated strings from the language pack
    pub translated_string_count: i32,
    /// Total number of non-deleted strings from the language pack available locally
    pub local_string_count: i32,
    /// Link to language translation interface; empty for custom local language packs
    pub translation_url: String,
}
