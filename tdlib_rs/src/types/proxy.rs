#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a proxy server
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Proxy {
    /// Proxy server domain or IP address
    pub server: String,
    /// Proxy server port
    pub port: i32,
    /// Type of the proxy
    pub r#type: crate::enums::ProxyType,
}
