#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a fact-check added to the message by an independent checker
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FactCheck {
    /// Text of the fact-check
    pub text: crate::types::FormattedText,
    /// A two-letter ISO 3166-1 alpha-2 country code of the country for which the fact-check is shown
    pub country_code: String,
}
