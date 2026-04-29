#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPublicPosts {
    /// Contains a list of messages found by a public post search
    #[serde(rename(serialize = "foundPublicPosts", deserialize = "foundPublicPosts"))]
    FoundPublicPosts(crate::types::FoundPublicPosts),
}
