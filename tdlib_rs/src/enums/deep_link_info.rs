#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DeepLinkInfo {
    /// Contains information about a tg: deep link
    #[serde(rename(serialize = "deepLinkInfo", deserialize = "deepLinkInfo"))]
    DeepLinkInfo(crate::types::DeepLinkInfo),
}
