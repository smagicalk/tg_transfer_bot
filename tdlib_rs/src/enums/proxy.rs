#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Proxy {
    /// Describes a proxy server
    #[serde(rename(serialize = "proxy", deserialize = "proxy"))]
    Proxy(crate::types::Proxy),
}
