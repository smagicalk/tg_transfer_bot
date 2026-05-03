#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanTransferOwnershipResult {
    /// The session can be used
    #[serde(rename(
        serialize = "canTransferOwnershipResultOk",
        deserialize = "canTransferOwnershipResultOk"
    ))]
    Ok,
    /// The 2-step verification needs to be enabled first
    #[serde(rename(
        serialize = "canTransferOwnershipResultPasswordNeeded",
        deserialize = "canTransferOwnershipResultPasswordNeeded"
    ))]
    PasswordNeeded,
    /// The 2-step verification was enabled recently, user needs to wait
    #[serde(rename(
        serialize = "canTransferOwnershipResultPasswordTooFresh",
        deserialize = "canTransferOwnershipResultPasswordTooFresh"
    ))]
    PasswordTooFresh(crate::types::CanTransferOwnershipResultPasswordTooFresh),
    /// The session was created recently, user needs to wait
    #[serde(rename(
        serialize = "canTransferOwnershipResultSessionTooFresh",
        deserialize = "canTransferOwnershipResultSessionTooFresh"
    ))]
    SessionTooFresh(crate::types::CanTransferOwnershipResultSessionTooFresh),
}
