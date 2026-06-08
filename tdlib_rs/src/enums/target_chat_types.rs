#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TargetChatTypes {
    /// Describes allowed types for the target chat
    #[serde(rename(serialize = "targetChatTypes", deserialize = "targetChatTypes"))]
    TargetChatTypes(crate::types::TargetChatTypes),
}
