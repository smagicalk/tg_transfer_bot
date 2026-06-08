#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains statistics about administrator actions done by a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsAdministratorActionsInfo {
    /// Administrator user identifier
    pub user_id: i64,
    /// Number of messages deleted by the administrator
    pub deleted_message_count: i32,
    /// Number of users banned by the administrator
    pub banned_user_count: i32,
    /// Number of users restricted by the administrator
    pub restricted_user_count: i32,
}
