#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredChat {
    /// Describes a sponsored chat
    #[serde(rename(serialize = "sponsoredChat", deserialize = "sponsoredChat"))]
    SponsoredChat(crate::types::SponsoredChat),
}
