#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResults {
    /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
    #[serde(rename(serialize = "inlineQueryResults", deserialize = "inlineQueryResults"))]
    InlineQueryResults(crate::types::InlineQueryResults),
}
