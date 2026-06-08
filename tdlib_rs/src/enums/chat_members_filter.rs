#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMembersFilter {
    /// Returns contacts of the user
    #[serde(rename(
        serialize = "chatMembersFilterContacts",
        deserialize = "chatMembersFilterContacts"
    ))]
    Contacts,
    /// Returns the owner and administrators
    #[serde(rename(
        serialize = "chatMembersFilterAdministrators",
        deserialize = "chatMembersFilterAdministrators"
    ))]
    Administrators,
    /// Returns all chat members, including restricted chat members
    #[serde(rename(
        serialize = "chatMembersFilterMembers",
        deserialize = "chatMembersFilterMembers"
    ))]
    Members,
    /// Returns users which can be mentioned in the chat
    #[serde(rename(
        serialize = "chatMembersFilterMention",
        deserialize = "chatMembersFilterMention"
    ))]
    Mention(crate::types::ChatMembersFilterMention),
    /// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
    #[serde(rename(
        serialize = "chatMembersFilterRestricted",
        deserialize = "chatMembersFilterRestricted"
    ))]
    Restricted,
    /// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
    #[serde(rename(
        serialize = "chatMembersFilterBanned",
        deserialize = "chatMembersFilterBanned"
    ))]
    Banned,
    /// Returns bot members of the chat
    #[serde(rename(
        serialize = "chatMembersFilterBots",
        deserialize = "chatMembersFilterBots"
    ))]
    Bots,
}
