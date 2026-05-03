#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Backgrounds {
    /// Contains a list of backgrounds
    #[serde(rename(serialize = "backgrounds", deserialize = "backgrounds"))]
    Backgrounds(crate::types::Backgrounds),
}
