#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredChats {
    /// Contains a list of sponsored chats
    #[serde(rename(serialize = "sponsoredChats", deserialize = "sponsoredChats"))]
    SponsoredChats(crate::types::SponsoredChats),
}
