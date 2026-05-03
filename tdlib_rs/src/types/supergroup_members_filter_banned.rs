#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Returns users banned from the supergroup or channel; can be used only by administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterBanned {
    /// Query to search for
    pub query: String,
}
