#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySettingRules {
    /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
    #[serde(rename(
        serialize = "userPrivacySettingRules",
        deserialize = "userPrivacySettingRules"
    ))]
    UserPrivacySettingRules(crate::types::UserPrivacySettingRules),
}
