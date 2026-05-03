#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of messages found by a public post search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundPublicPosts {
    /// List of found public posts
    pub messages: Vec<crate::types::Message>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
    /// Updated public post search limits after the query; repeated requests with the same query will be free; may be null if they didn't change
    pub search_limits: Option<crate::types::PublicPostSearchLimits>,
    /// True, if the query has failed because search limits are exceeded. In this case search_limits.daily_free_query_count will be equal to 0
    pub are_limits_exceeded: bool,
}
