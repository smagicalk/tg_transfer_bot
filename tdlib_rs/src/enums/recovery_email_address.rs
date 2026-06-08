#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecoveryEmailAddress {
    /// Contains information about the current recovery email address
    #[serde(rename(
        serialize = "recoveryEmailAddress",
        deserialize = "recoveryEmailAddress"
    ))]
    RecoveryEmailAddress(crate::types::RecoveryEmailAddress),
}
