#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStringValue {
    /// An ordinary language pack string
    #[serde(rename(serialize = "languagePackStringValueOrdinary", deserialize = "languagePackStringValueOrdinary"))]
    Ordinary(crate::types::LanguagePackStringValueOrdinary),
    /// A language pack string which has different forms based on the number of some object it mentions. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
    #[serde(rename(serialize = "languagePackStringValuePluralized", deserialize = "languagePackStringValuePluralized"))]
    Pluralized(crate::types::LanguagePackStringValuePluralized),
    /// A deleted language pack string, the value must be taken from the built-in English language pack
    #[serde(rename(serialize = "languagePackStringValueDeleted", deserialize = "languagePackStringValueDeleted"))]
    Deleted,
}
