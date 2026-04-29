#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SecretChat {
    /// Represents a secret chat
    #[serde(rename(serialize = "secretChat", deserialize = "secretChat"))]
    SecretChat(crate::types::SecretChat),
}
