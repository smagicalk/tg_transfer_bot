#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResultsButton {
    /// Represents a button to be shown above inline query results
    #[serde(rename(
        serialize = "inlineQueryResultsButton",
        deserialize = "inlineQueryResultsButton"
    ))]
    InlineQueryResultsButton(crate::types::InlineQueryResultsButton),
}
