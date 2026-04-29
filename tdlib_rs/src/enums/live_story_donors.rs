#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LiveStoryDonors {
    /// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
    #[serde(rename(serialize = "liveStoryDonors", deserialize = "liveStoryDonors"))]
    LiveStoryDonors(crate::types::LiveStoryDonors),
}
