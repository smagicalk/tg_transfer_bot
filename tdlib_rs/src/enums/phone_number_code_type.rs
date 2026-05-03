#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberCodeType {
    /// Checks ownership of a new phone number to change the user's authentication phone number; for official Android and iOS applications only
    #[serde(rename(
        serialize = "phoneNumberCodeTypeChange",
        deserialize = "phoneNumberCodeTypeChange"
    ))]
    Change,
    /// Verifies ownership of a phone number to be added to the user's Telegram Passport
    #[serde(rename(
        serialize = "phoneNumberCodeTypeVerify",
        deserialize = "phoneNumberCodeTypeVerify"
    ))]
    Verify,
    /// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
    #[serde(rename(
        serialize = "phoneNumberCodeTypeConfirmOwnership",
        deserialize = "phoneNumberCodeTypeConfirmOwnership"
    ))]
    ConfirmOwnership(crate::types::PhoneNumberCodeTypeConfirmOwnership),
}
