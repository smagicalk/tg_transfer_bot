#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeFoundVenue {
    /// Identifier of the inline query, used to found the venue
    #[serde_as(as = "DisplayFromStr")]
    pub query_id: i64,
    /// Identifier of the inline query result
    pub result_id: String,
}
