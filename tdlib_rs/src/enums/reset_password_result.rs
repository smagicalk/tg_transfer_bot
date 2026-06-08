#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ResetPasswordResult {
    /// The password was reset
    #[serde(rename(
        serialize = "resetPasswordResultOk",
        deserialize = "resetPasswordResultOk"
    ))]
    Ok,
    /// The password reset request is pending
    #[serde(rename(
        serialize = "resetPasswordResultPending",
        deserialize = "resetPasswordResultPending"
    ))]
    Pending(crate::types::ResetPasswordResultPending),
    /// The password reset request was declined
    #[serde(rename(
        serialize = "resetPasswordResultDeclined",
        deserialize = "resetPasswordResultDeclined"
    ))]
    Declined(crate::types::ResetPasswordResultDeclined),
}
