#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of sponsored chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SponsoredChats {
    /// List of sponsored chats
    pub chats: Vec<crate::types::SponsoredChat>,
}
