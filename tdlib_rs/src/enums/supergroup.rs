#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Supergroup {
    /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup:
    /// only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos.
    /// Unlike supergroups, channels can have an unlimited number of subscribers
    #[serde(rename(serialize = "supergroup", deserialize = "supergroup"))]
    Supergroup(crate::types::Supergroup),
}
