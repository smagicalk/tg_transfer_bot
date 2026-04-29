#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VerificationStatus {
    /// Contains information about verification status of a chat or a user
    #[serde(rename(serialize = "verificationStatus", deserialize = "verificationStatus"))]
    VerificationStatus(crate::types::VerificationStatus),
}
