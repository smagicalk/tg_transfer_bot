#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForwardSource {
    /// Contains information about the last message from which a new message was forwarded last time
    #[serde(rename(serialize = "forwardSource", deserialize = "forwardSource"))]
    ForwardSource(crate::types::ForwardSource),
}
