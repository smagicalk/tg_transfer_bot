#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of found users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundUsers {
    /// Identifiers of the found users
    pub user_ids: Vec<i64>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}
