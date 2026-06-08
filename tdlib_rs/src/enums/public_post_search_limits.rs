#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicPostSearchLimits {
    /// Contains information about public post search limits
    #[serde(rename(
        serialize = "publicPostSearchLimits",
        deserialize = "publicPostSearchLimits"
    ))]
    PublicPostSearchLimits(crate::types::PublicPostSearchLimits),
}
