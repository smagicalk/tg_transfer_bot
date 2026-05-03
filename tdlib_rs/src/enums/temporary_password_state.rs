#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TemporaryPasswordState {
    /// Returns information about the availability of a temporary password, which can be used for payments
    #[serde(rename(
        serialize = "temporaryPasswordState",
        deserialize = "temporaryPasswordState"
    ))]
    TemporaryPasswordState(crate::types::TemporaryPasswordState),
}
