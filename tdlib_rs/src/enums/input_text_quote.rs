#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputTextQuote {
    /// Describes manually chosen quote from another message
    #[serde(rename(serialize = "inputTextQuote", deserialize = "inputTextQuote"))]
    InputTextQuote(crate::types::InputTextQuote),
}
