#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Gift {
    /// Describes a gift that can be sent to another user or channel chat
    #[serde(rename(serialize = "gift", deserialize = "gift"))]
    Gift(crate::types::Gift),
}
