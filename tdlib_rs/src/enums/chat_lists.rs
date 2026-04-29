#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatLists {
    /// Contains a list of chat lists
    #[serde(rename(serialize = "chatLists", deserialize = "chatLists"))]
    ChatLists(crate::types::ChatLists),
}
