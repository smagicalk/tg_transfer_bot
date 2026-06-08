#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A rule to allow all members of certain specified basic groups and supergroups to doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleAllowChatMembers {
    /// The chat identifiers, total number of chats in all rules must not exceed 20
    pub chat_ids: Vec<i64>,
}
