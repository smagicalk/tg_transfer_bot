#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Passkeys {
    /// Contains a list of passkeys
    #[serde(rename(serialize = "passkeys", deserialize = "passkeys"))]
    Passkeys(crate::types::Passkeys),
}
