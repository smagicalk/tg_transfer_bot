#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedProxies {
    /// Represents a list of added proxy servers
    #[serde(rename(serialize = "addedProxies", deserialize = "addedProxies"))]
    AddedProxies(crate::types::AddedProxies),
}
