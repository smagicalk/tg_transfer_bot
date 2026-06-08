#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BlockList {
    /// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
    #[serde(rename(serialize = "blockListMain", deserialize = "blockListMain"))]
    Main,
    /// The block list that disallows viewing of stories of the current user
    #[serde(rename(serialize = "blockListStories", deserialize = "blockListStories"))]
    Stories,
}
