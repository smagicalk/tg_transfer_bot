#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkType {
    /// The network is not available
    #[serde(rename(serialize = "networkTypeNone", deserialize = "networkTypeNone"))]
    None,
    /// A mobile network
    #[serde(rename(serialize = "networkTypeMobile", deserialize = "networkTypeMobile"))]
    Mobile,
    /// A mobile roaming network
    #[serde(rename(serialize = "networkTypeMobileRoaming", deserialize = "networkTypeMobileRoaming"))]
    MobileRoaming,
    /// A Wi-Fi network
    #[serde(rename(serialize = "networkTypeWiFi", deserialize = "networkTypeWiFi"))]
    WiFi,
    /// A different network type (e.g., Ethernet network)
    #[serde(rename(serialize = "networkTypeOther", deserialize = "networkTypeOther"))]
    Other,
}
