#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FactCheck {
    /// Describes a fact-check added to the message by an independent checker
    #[serde(rename(serialize = "factCheck", deserialize = "factCheck"))]
    FactCheck(crate::types::FactCheck),
}
