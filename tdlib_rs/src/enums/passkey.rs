#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Passkey {
    /// Describes a passkey
    #[serde(rename(serialize = "passkey", deserialize = "passkey"))]
    Passkey(crate::types::Passkey),
}
