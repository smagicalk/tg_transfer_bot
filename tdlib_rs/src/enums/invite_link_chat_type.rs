#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InviteLinkChatType {
    /// The link is an invite link for a basic group
    #[serde(rename(
        serialize = "inviteLinkChatTypeBasicGroup",
        deserialize = "inviteLinkChatTypeBasicGroup"
    ))]
    BasicGroup,
    /// The link is an invite link for a supergroup
    #[serde(rename(
        serialize = "inviteLinkChatTypeSupergroup",
        deserialize = "inviteLinkChatTypeSupergroup"
    ))]
    Supergroup,
    /// The link is an invite link for a channel
    #[serde(rename(
        serialize = "inviteLinkChatTypeChannel",
        deserialize = "inviteLinkChatTypeChannel"
    ))]
    Channel,
}
