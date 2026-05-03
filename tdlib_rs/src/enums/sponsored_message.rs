#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredMessage {
    /// Describes a sponsored message
    #[serde(rename(serialize = "sponsoredMessage", deserialize = "sponsoredMessage"))]
    SponsoredMessage(crate::types::SponsoredMessage),
}
