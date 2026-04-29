#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundChatBoosts {
    /// Contains a list of boosts applied to a chat
    #[serde(rename(serialize = "foundChatBoosts", deserialize = "foundChatBoosts"))]
    FoundChatBoosts(crate::types::FoundChatBoosts),
}
