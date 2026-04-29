#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBusinessChatLink {
    /// Describes a business chat link to create or edit
    #[serde(rename(serialize = "inputBusinessChatLink", deserialize = "inputBusinessChatLink"))]
    InputBusinessChatLink(crate::types::InputBusinessChatLink),
}
