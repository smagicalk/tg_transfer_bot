#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A rule to restrict all specified users from doing something
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserPrivacySettingRuleRestrictUsers {
    /// The user identifiers, total number of users in all rules must not exceed 1000
    pub user_ids: Vec<i64>,
}
