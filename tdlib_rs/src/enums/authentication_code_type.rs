#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeType {
    /// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
    #[serde(rename(
        serialize = "authenticationCodeTypeTelegramMessage",
        deserialize = "authenticationCodeTypeTelegramMessage"
    ))]
    TelegramMessage(crate::types::AuthenticationCodeTypeTelegramMessage),
    /// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(
        serialize = "authenticationCodeTypeSms",
        deserialize = "authenticationCodeTypeSms"
    ))]
    Sms(crate::types::AuthenticationCodeTypeSms),
    /// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(
        serialize = "authenticationCodeTypeSmsWord",
        deserialize = "authenticationCodeTypeSmsWord"
    ))]
    SmsWord(crate::types::AuthenticationCodeTypeSmsWord),
    /// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(
        serialize = "authenticationCodeTypeSmsPhrase",
        deserialize = "authenticationCodeTypeSmsPhrase"
    ))]
    SmsPhrase(crate::types::AuthenticationCodeTypeSmsPhrase),
    /// A digit-only authentication code is delivered via a phone call to the specified phone number
    #[serde(rename(
        serialize = "authenticationCodeTypeCall",
        deserialize = "authenticationCodeTypeCall"
    ))]
    Call(crate::types::AuthenticationCodeTypeCall),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
    #[serde(rename(
        serialize = "authenticationCodeTypeFlashCall",
        deserialize = "authenticationCodeTypeFlashCall"
    ))]
    FlashCall(crate::types::AuthenticationCodeTypeFlashCall),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
    #[serde(rename(
        serialize = "authenticationCodeTypeMissedCall",
        deserialize = "authenticationCodeTypeMissedCall"
    ))]
    MissedCall(crate::types::AuthenticationCodeTypeMissedCall),
    /// A digit-only authentication code is delivered to https:fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
    #[serde(rename(
        serialize = "authenticationCodeTypeFragment",
        deserialize = "authenticationCodeTypeFragment"
    ))]
    Fragment(crate::types::AuthenticationCodeTypeFragment),
    /// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
    #[serde(rename(
        serialize = "authenticationCodeTypeFirebaseAndroid",
        deserialize = "authenticationCodeTypeFirebaseAndroid"
    ))]
    FirebaseAndroid(crate::types::AuthenticationCodeTypeFirebaseAndroid),
    /// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
    #[serde(rename(
        serialize = "authenticationCodeTypeFirebaseIos",
        deserialize = "authenticationCodeTypeFirebaseIos"
    ))]
    FirebaseIos(crate::types::AuthenticationCodeTypeFirebaseIos),
}
