#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SupergroupMembersFilter {
    /// Returns recently active users in reverse chronological order
    #[serde(rename(
        serialize = "supergroupMembersFilterRecent",
        deserialize = "supergroupMembersFilterRecent"
    ))]
    Recent,
    /// Returns contacts of the user, which are members of the supergroup or channel
    #[serde(rename(
        serialize = "supergroupMembersFilterContacts",
        deserialize = "supergroupMembersFilterContacts"
    ))]
    Contacts(crate::types::SupergroupMembersFilterContacts),
    /// Returns the owner and administrators
    #[serde(rename(
        serialize = "supergroupMembersFilterAdministrators",
        deserialize = "supergroupMembersFilterAdministrators"
    ))]
    Administrators,
    /// Used to search for supergroup or channel members via a (string) query
    #[serde(rename(
        serialize = "supergroupMembersFilterSearch",
        deserialize = "supergroupMembersFilterSearch"
    ))]
    Search(crate::types::SupergroupMembersFilterSearch),
    /// Returns restricted supergroup members; can be used only by administrators
    #[serde(rename(
        serialize = "supergroupMembersFilterRestricted",
        deserialize = "supergroupMembersFilterRestricted"
    ))]
    Restricted(crate::types::SupergroupMembersFilterRestricted),
    /// Returns users banned from the supergroup or channel; can be used only by administrators
    #[serde(rename(
        serialize = "supergroupMembersFilterBanned",
        deserialize = "supergroupMembersFilterBanned"
    ))]
    Banned(crate::types::SupergroupMembersFilterBanned),
    /// Returns users which can be mentioned in the supergroup
    #[serde(rename(
        serialize = "supergroupMembersFilterMention",
        deserialize = "supergroupMembersFilterMention"
    ))]
    Mention(crate::types::SupergroupMembersFilterMention),
    /// Returns bot members of the supergroup or channel
    #[serde(rename(
        serialize = "supergroupMembersFilterBots",
        deserialize = "supergroupMembersFilterBots"
    ))]
    Bots,
}
