#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An ordinary language pack string
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LanguagePackStringValueOrdinary {
    /// String value
    pub value: String,
}
