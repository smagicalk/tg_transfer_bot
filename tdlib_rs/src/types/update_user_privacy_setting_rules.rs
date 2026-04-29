#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some privacy setting rules have been changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateUserPrivacySettingRules {
    /// The privacy setting
    pub setting: crate::enums::UserPrivacySetting,
    /// New privacy rules
    pub rules: crate::types::UserPrivacySettingRules,
}
