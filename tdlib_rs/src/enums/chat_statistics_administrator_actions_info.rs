#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsAdministratorActionsInfo {
    /// Contains statistics about administrator actions done by a user
    #[serde(rename(serialize = "chatStatisticsAdministratorActionsInfo", deserialize = "chatStatisticsAdministratorActionsInfo"))]
    ChatStatisticsAdministratorActionsInfo(crate::types::ChatStatisticsAdministratorActionsInfo),
}
