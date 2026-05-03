#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedAffiliatePrograms {
    /// Represents a list of affiliate programs that were connected to an affiliate
    #[serde(rename(
        serialize = "connectedAffiliatePrograms",
        deserialize = "connectedAffiliatePrograms"
    ))]
    ConnectedAffiliatePrograms(crate::types::ConnectedAffiliatePrograms),
}
