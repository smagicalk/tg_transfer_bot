#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a proxy server added to the list of proxies
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddedProxy {
    /// Unique identifier of the proxy
    pub id: i32,
    /// Point in time (Unix timestamp) when the proxy was last used; 0 if never
    pub last_used_date: i32,
    /// True, if the proxy is enabled now
    pub is_enabled: bool,
    /// The proxy
    pub proxy: crate::types::Proxy,
}
