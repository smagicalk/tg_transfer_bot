#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResults {
    /// Unique identifier of the inline query
    #[serde_as(as = "DisplayFromStr")]
    pub inline_query_id: i64,
    /// Button to be shown above inline query results; may be null
    pub button: Option<crate::types::InlineQueryResultsButton>,
    /// Results of the query
    pub results: Vec<crate::enums::InlineQueryResult>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
