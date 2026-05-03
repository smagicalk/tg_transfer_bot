#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PasswordState {
    /// Represents the current state of 2-step verification
    #[serde(rename(serialize = "passwordState", deserialize = "passwordState"))]
    PasswordState(crate::types::PasswordState),
}
