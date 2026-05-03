#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextQuote {
    /// Describes manually or automatically chosen quote from another message
    #[serde(rename(serialize = "textQuote", deserialize = "textQuote"))]
    TextQuote(crate::types::TextQuote),
}
