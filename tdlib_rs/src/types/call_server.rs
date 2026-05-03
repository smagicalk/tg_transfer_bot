#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a server for relaying call data
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CallServer {
    /// Server identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// Server IPv4 address
    pub ip_address: String,
    /// Server IPv6 address
    pub ipv6_address: String,
    /// Server port number
    pub port: i32,
    /// Server type
    pub r#type: crate::enums::CallServerType,
}
