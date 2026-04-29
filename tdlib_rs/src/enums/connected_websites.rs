#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedWebsites {
    /// Contains a list of websites the current user is logged in with Telegram
    #[serde(rename(serialize = "connectedWebsites", deserialize = "connectedWebsites"))]
    ConnectedWebsites(crate::types::ConnectedWebsites),
}
