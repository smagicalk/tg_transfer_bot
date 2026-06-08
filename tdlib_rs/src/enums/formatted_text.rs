#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FormattedText {
    /// A text with some entities
    #[serde(rename(serialize = "formattedText", deserialize = "formattedText"))]
    FormattedText(crate::types::FormattedText),
}
