#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberAuthenticationSettings {
    /// Contains settings for the authentication of the user's phone number
    #[serde(rename(
        serialize = "phoneNumberAuthenticationSettings",
        deserialize = "phoneNumberAuthenticationSettings"
    ))]
    PhoneNumberAuthenticationSettings(crate::types::PhoneNumberAuthenticationSettings),
}
