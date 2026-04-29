#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchMessagesChatTypeFilter {
    /// Returns only messages in private chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterPrivate", deserialize = "searchMessagesChatTypeFilterPrivate"))]
    Private,
    /// Returns only messages in basic group and supergroup chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterGroup", deserialize = "searchMessagesChatTypeFilterGroup"))]
    Group,
    /// Returns only messages in channel chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterChannel", deserialize = "searchMessagesChatTypeFilterChannel"))]
    Channel,
}
