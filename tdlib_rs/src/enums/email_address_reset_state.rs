#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressResetState {
    /// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
    #[serde(rename(
        serialize = "emailAddressResetStateAvailable",
        deserialize = "emailAddressResetStateAvailable"
    ))]
    Available(crate::types::EmailAddressResetStateAvailable),
    /// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
    #[serde(rename(
        serialize = "emailAddressResetStatePending",
        deserialize = "emailAddressResetStatePending"
    ))]
    Pending(crate::types::EmailAddressResetStatePending),
}
