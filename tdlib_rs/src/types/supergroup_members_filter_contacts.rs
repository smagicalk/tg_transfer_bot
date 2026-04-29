#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Returns contacts of the user, which are members of the supergroup or channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SupergroupMembersFilterContacts {
    /// Query to search for
    pub query: String,
}
