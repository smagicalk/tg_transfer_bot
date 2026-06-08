#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostState {
    /// The post must be approved or declined
    #[serde(rename(
        serialize = "suggestedPostStatePending",
        deserialize = "suggestedPostStatePending"
    ))]
    Pending,
    /// The post was approved
    #[serde(rename(
        serialize = "suggestedPostStateApproved",
        deserialize = "suggestedPostStateApproved"
    ))]
    Approved,
    /// The post was declined
    #[serde(rename(
        serialize = "suggestedPostStateDeclined",
        deserialize = "suggestedPostStateDeclined"
    ))]
    Declined,
}
