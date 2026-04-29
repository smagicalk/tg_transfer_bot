#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about the total amount of data that was used to send and receive files
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NetworkStatisticsEntryFile {
    /// Type of the file the data is part of; pass null if the data isn't related to files
    pub file_type: Option<crate::enums::FileType>,
    /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type
    pub network_type: crate::enums::NetworkType,
    /// Total number of bytes sent
    pub sent_bytes: i64,
    /// Total number of bytes received
    pub received_bytes: i64,
}
