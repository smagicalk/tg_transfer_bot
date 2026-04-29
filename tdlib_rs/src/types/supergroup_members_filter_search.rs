#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Used to search for supergroup or channel members via a (string) query
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterSearch {
    /// Query to search for
    pub query: String,
}
