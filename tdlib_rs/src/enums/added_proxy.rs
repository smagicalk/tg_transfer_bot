#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedProxy {
    /// Contains information about a proxy server added to the list of proxies
    #[serde(rename(serialize = "addedProxy", deserialize = "addedProxy"))]
    AddedProxy(crate::types::AddedProxy),
}
