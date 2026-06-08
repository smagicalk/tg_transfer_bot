#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkStatisticsEntry {
    /// Contains information about the total amount of data that was used to send and receive files
    #[serde(rename(
        serialize = "networkStatisticsEntryFile",
        deserialize = "networkStatisticsEntryFile"
    ))]
    File(crate::types::NetworkStatisticsEntryFile),
    /// Contains information about the total amount of data that was used for calls
    #[serde(rename(
        serialize = "networkStatisticsEntryCall",
        deserialize = "networkStatisticsEntryCall"
    ))]
    Call(crate::types::NetworkStatisticsEntryCall),
}
