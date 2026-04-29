#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectionState {
    /// Waiting for the network to become available. Use setNetworkType to change the available network type
    #[serde(rename(serialize = "connectionStateWaitingForNetwork", deserialize = "connectionStateWaitingForNetwork"))]
    WaitingForNetwork,
    /// Establishing a connection with a proxy server
    #[serde(rename(serialize = "connectionStateConnectingToProxy", deserialize = "connectionStateConnectingToProxy"))]
    ConnectingToProxy,
    /// Establishing a connection to the Telegram servers
    #[serde(rename(serialize = "connectionStateConnecting", deserialize = "connectionStateConnecting"))]
    Connecting,
    /// Downloading data expected to be received while the application was offline
    #[serde(rename(serialize = "connectionStateUpdating", deserialize = "connectionStateUpdating"))]
    Updating,
    /// There is a working connection to the Telegram servers
    #[serde(rename(serialize = "connectionStateReady", deserialize = "connectionStateReady"))]
    Ready,
}
