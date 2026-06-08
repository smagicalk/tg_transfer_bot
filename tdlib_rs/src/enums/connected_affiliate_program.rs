#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedAffiliateProgram {
    /// Describes an affiliate program that was connected to an affiliate
    #[serde(rename(
        serialize = "connectedAffiliateProgram",
        deserialize = "connectedAffiliateProgram"
    ))]
    ConnectedAffiliateProgram(crate::types::ConnectedAffiliateProgram),
}
