#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySettingRule {
    /// A rule to allow all users to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowAll",
        deserialize = "userPrivacySettingRuleAllowAll"
    ))]
    AllowAll,
    /// A rule to allow all contacts of the user to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowContacts",
        deserialize = "userPrivacySettingRuleAllowContacts"
    ))]
    AllowContacts,
    /// A rule to allow all bots to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowBots",
        deserialize = "userPrivacySettingRuleAllowBots"
    ))]
    AllowBots,
    /// A rule to allow all Premium Users to do something; currently, allowed only for userPrivacySettingAllowChatInvites
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowPremiumUsers",
        deserialize = "userPrivacySettingRuleAllowPremiumUsers"
    ))]
    AllowPremiumUsers,
    /// A rule to allow certain specified users to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowUsers",
        deserialize = "userPrivacySettingRuleAllowUsers"
    ))]
    AllowUsers(crate::types::UserPrivacySettingRuleAllowUsers),
    /// A rule to allow all members of certain specified basic groups and supergroups to doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowChatMembers",
        deserialize = "userPrivacySettingRuleAllowChatMembers"
    ))]
    AllowChatMembers(crate::types::UserPrivacySettingRuleAllowChatMembers),
    /// A rule to restrict all users from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictAll",
        deserialize = "userPrivacySettingRuleRestrictAll"
    ))]
    RestrictAll,
    /// A rule to restrict all contacts of the user from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictContacts",
        deserialize = "userPrivacySettingRuleRestrictContacts"
    ))]
    RestrictContacts,
    /// A rule to restrict all bots from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictBots",
        deserialize = "userPrivacySettingRuleRestrictBots"
    ))]
    RestrictBots,
    /// A rule to restrict all specified users from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictUsers",
        deserialize = "userPrivacySettingRuleRestrictUsers"
    ))]
    RestrictUsers(crate::types::UserPrivacySettingRuleRestrictUsers),
    /// A rule to restrict all members of specified basic groups and supergroups from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictChatMembers",
        deserialize = "userPrivacySettingRuleRestrictChatMembers"
    ))]
    RestrictChatMembers(crate::types::UserPrivacySettingRuleRestrictChatMembers),
}
