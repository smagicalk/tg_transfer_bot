#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredMessages {
    /// Contains a list of sponsored messages
    #[serde(rename(serialize = "sponsoredMessages", deserialize = "sponsoredMessages"))]
    SponsoredMessages(crate::types::SponsoredMessages),
}
