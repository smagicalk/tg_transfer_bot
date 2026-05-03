#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedWebsite {
    /// Contains information about one website the current user is logged in with Telegram
    #[serde(rename(serialize = "connectedWebsite", deserialize = "connectedWebsite"))]
    ConnectedWebsite(crate::types::ConnectedWebsite),
}
