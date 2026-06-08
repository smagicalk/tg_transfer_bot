#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Animations {
    /// Represents a list of animations
    #[serde(rename(serialize = "animations", deserialize = "animations"))]
    Animations(crate::types::Animations),
}
