#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GroupCallVideoSourceGroup {
    /// Describes a group of video synchronization source identifiers
    #[serde(rename(
        serialize = "groupCallVideoSourceGroup",
        deserialize = "groupCallVideoSourceGroup"
    ))]
    GroupCallVideoSourceGroup(crate::types::GroupCallVideoSourceGroup),
}
