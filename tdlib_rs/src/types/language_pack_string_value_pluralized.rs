#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A language pack string which has different forms based on the number of some object it mentions. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStringValuePluralized {
    /// Value for zero objects
    pub zero_value: String,
    /// Value for one object
    pub one_value: String,
    /// Value for two objects
    pub two_value: String,
    /// Value for few objects
    pub few_value: String,
    /// Value for many objects
    pub many_value: String,
    /// Default value
    pub other_value: String,
}
