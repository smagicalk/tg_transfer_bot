//!
//! TDLib `misc` domain enums.
//!
//! Miscellaneous types, enums, network settings, and core TDLib options.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `Error` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Error {
    /// An object of this type can be returned on every function call, in case of an error
    #[serde(rename(serialize = "error", deserialize = "error"))]
    Error(Box<crate::types::Error>),
}

impl Error {
    /// Convenience constructor to create a [`Error::Error`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn error(val: crate::types::Error) -> Self {
        Self::Error(Box::new(val))
    }

}

/// Converts a [`crate::types::Error`] into [`Error`].
impl From<crate::types::Error> for Error {
    fn from(val: crate::types::Error) -> Self {
        Self::Error(Box::new(val))
    }
}

/// Provides information about the method by which an authentication code is delivered to the user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeType {
    /// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
    #[serde(rename(serialize = "authenticationCodeTypeTelegramMessage", deserialize = "authenticationCodeTypeTelegramMessage"))]
    TelegramMessage(Box<crate::types::AuthenticationCodeTypeTelegramMessage>),
    /// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(serialize = "authenticationCodeTypeSms", deserialize = "authenticationCodeTypeSms"))]
    Sms(Box<crate::types::AuthenticationCodeTypeSms>),
    /// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(serialize = "authenticationCodeTypeSmsWord", deserialize = "authenticationCodeTypeSmsWord"))]
    SmsWord(Box<crate::types::AuthenticationCodeTypeSmsWord>),
    /// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
    #[serde(rename(serialize = "authenticationCodeTypeSmsPhrase", deserialize = "authenticationCodeTypeSmsPhrase"))]
    SmsPhrase(Box<crate::types::AuthenticationCodeTypeSmsPhrase>),
    /// A digit-only authentication code is delivered via a phone call to the specified phone number
    #[serde(rename(serialize = "authenticationCodeTypeCall", deserialize = "authenticationCodeTypeCall"))]
    Call(Box<crate::types::AuthenticationCodeTypeCall>),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
    #[serde(rename(serialize = "authenticationCodeTypeFlashCall", deserialize = "authenticationCodeTypeFlashCall"))]
    FlashCall(Box<crate::types::AuthenticationCodeTypeFlashCall>),
    /// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
    #[serde(rename(serialize = "authenticationCodeTypeMissedCall", deserialize = "authenticationCodeTypeMissedCall"))]
    MissedCall(Box<crate::types::AuthenticationCodeTypeMissedCall>),
    /// A digit-only authentication code is delivered to https:fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
    #[serde(rename(serialize = "authenticationCodeTypeFragment", deserialize = "authenticationCodeTypeFragment"))]
    Fragment(Box<crate::types::AuthenticationCodeTypeFragment>),
    /// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
    #[serde(rename(serialize = "authenticationCodeTypeFirebaseAndroid", deserialize = "authenticationCodeTypeFirebaseAndroid"))]
    FirebaseAndroid(Box<crate::types::AuthenticationCodeTypeFirebaseAndroid>),
    /// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
    #[serde(rename(serialize = "authenticationCodeTypeFirebaseIos", deserialize = "authenticationCodeTypeFirebaseIos"))]
    FirebaseIos(Box<crate::types::AuthenticationCodeTypeFirebaseIos>),
}

impl AuthenticationCodeType {
    /// Convenience constructor to create a [`AuthenticationCodeType::TelegramMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn telegram_message(val: crate::types::AuthenticationCodeTypeTelegramMessage) -> Self {
        Self::TelegramMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::Sms`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sms(val: crate::types::AuthenticationCodeTypeSms) -> Self {
        Self::Sms(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::SmsWord`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sms_word(val: crate::types::AuthenticationCodeTypeSmsWord) -> Self {
        Self::SmsWord(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::SmsPhrase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sms_phrase(val: crate::types::AuthenticationCodeTypeSmsPhrase) -> Self {
        Self::SmsPhrase(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::Call`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call(val: crate::types::AuthenticationCodeTypeCall) -> Self {
        Self::Call(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::FlashCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn flash_call(val: crate::types::AuthenticationCodeTypeFlashCall) -> Self {
        Self::FlashCall(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::MissedCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn missed_call(val: crate::types::AuthenticationCodeTypeMissedCall) -> Self {
        Self::MissedCall(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::Fragment`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment(val: crate::types::AuthenticationCodeTypeFragment) -> Self {
        Self::Fragment(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::FirebaseAndroid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn firebase_android(val: crate::types::AuthenticationCodeTypeFirebaseAndroid) -> Self {
        Self::FirebaseAndroid(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthenticationCodeType::FirebaseIos`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn firebase_ios(val: crate::types::AuthenticationCodeTypeFirebaseIos) -> Self {
        Self::FirebaseIos(Box::new(val))
    }

}

/// Converts a [`crate::types::AuthenticationCodeTypeTelegramMessage`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeTelegramMessage> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeTelegramMessage) -> Self {
        Self::TelegramMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeSms`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeSms> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeSms) -> Self {
        Self::Sms(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeSmsWord`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeSmsWord> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeSmsWord) -> Self {
        Self::SmsWord(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeSmsPhrase`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeSmsPhrase> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeSmsPhrase) -> Self {
        Self::SmsPhrase(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeCall`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeCall> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeCall) -> Self {
        Self::Call(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeFlashCall`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeFlashCall> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeFlashCall) -> Self {
        Self::FlashCall(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeMissedCall`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeMissedCall> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeMissedCall) -> Self {
        Self::MissedCall(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeFragment`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeFragment> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeFragment) -> Self {
        Self::Fragment(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeFirebaseAndroid`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeFirebaseAndroid> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeFirebaseAndroid) -> Self {
        Self::FirebaseAndroid(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthenticationCodeTypeFirebaseIos`] into [`AuthenticationCodeType`].
impl From<crate::types::AuthenticationCodeTypeFirebaseIos> for AuthenticationCodeType {
    fn from(val: crate::types::AuthenticationCodeTypeFirebaseIos) -> Self {
        Self::FirebaseIos(Box::new(val))
    }
}

/// TDLib `AuthenticationCodeInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthenticationCodeInfo {
    /// Information about the authentication code that was sent
    #[serde(rename(serialize = "authenticationCodeInfo", deserialize = "authenticationCodeInfo"))]
    AuthenticationCodeInfo(Box<crate::types::AuthenticationCodeInfo>),
}

impl AuthenticationCodeInfo {
    /// Convenience constructor to create a [`AuthenticationCodeInfo::AuthenticationCodeInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn authentication_code_info(val: crate::types::AuthenticationCodeInfo) -> Self {
        Self::AuthenticationCodeInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::AuthenticationCodeInfo`] into [`AuthenticationCodeInfo`].
impl From<crate::types::AuthenticationCodeInfo> for AuthenticationCodeInfo {
    fn from(val: crate::types::AuthenticationCodeInfo) -> Self {
        Self::AuthenticationCodeInfo(Box::new(val))
    }
}

/// TDLib `EmailAddressAuthenticationCodeInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressAuthenticationCodeInfo {
    /// Information about the email address authentication code that was sent
    #[serde(rename(serialize = "emailAddressAuthenticationCodeInfo", deserialize = "emailAddressAuthenticationCodeInfo"))]
    EmailAddressAuthenticationCodeInfo(Box<crate::types::EmailAddressAuthenticationCodeInfo>),
}

impl EmailAddressAuthenticationCodeInfo {
    /// Convenience constructor to create a [`EmailAddressAuthenticationCodeInfo::EmailAddressAuthenticationCodeInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn email_address_authentication_code_info(val: crate::types::EmailAddressAuthenticationCodeInfo) -> Self {
        Self::EmailAddressAuthenticationCodeInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::EmailAddressAuthenticationCodeInfo`] into [`EmailAddressAuthenticationCodeInfo`].
impl From<crate::types::EmailAddressAuthenticationCodeInfo> for EmailAddressAuthenticationCodeInfo {
    fn from(val: crate::types::EmailAddressAuthenticationCodeInfo) -> Self {
        Self::EmailAddressAuthenticationCodeInfo(Box::new(val))
    }
}

/// Contains authentication data for an email address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressAuthentication {
    /// An authentication code delivered to a user's email address
    #[serde(rename(serialize = "emailAddressAuthenticationCode", deserialize = "emailAddressAuthenticationCode"))]
    Code(Box<crate::types::EmailAddressAuthenticationCode>),
    /// An authentication token received through Apple ID
    #[serde(rename(serialize = "emailAddressAuthenticationAppleId", deserialize = "emailAddressAuthenticationAppleId"))]
    AppleId(Box<crate::types::EmailAddressAuthenticationAppleId>),
    /// An authentication token received through Google ID
    #[serde(rename(serialize = "emailAddressAuthenticationGoogleId", deserialize = "emailAddressAuthenticationGoogleId"))]
    GoogleId(Box<crate::types::EmailAddressAuthenticationGoogleId>),
}

impl EmailAddressAuthentication {
    /// Convenience constructor to create a [`EmailAddressAuthentication::Code`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn code(val: crate::types::EmailAddressAuthenticationCode) -> Self {
        Self::Code(Box::new(val))
    }

    /// Convenience constructor to create a [`EmailAddressAuthentication::AppleId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn apple_id(val: crate::types::EmailAddressAuthenticationAppleId) -> Self {
        Self::AppleId(Box::new(val))
    }

    /// Convenience constructor to create a [`EmailAddressAuthentication::GoogleId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn google_id(val: crate::types::EmailAddressAuthenticationGoogleId) -> Self {
        Self::GoogleId(Box::new(val))
    }

}

/// Converts a [`crate::types::EmailAddressAuthenticationCode`] into [`EmailAddressAuthentication`].
impl From<crate::types::EmailAddressAuthenticationCode> for EmailAddressAuthentication {
    fn from(val: crate::types::EmailAddressAuthenticationCode) -> Self {
        Self::Code(Box::new(val))
    }
}

/// Converts a [`crate::types::EmailAddressAuthenticationAppleId`] into [`EmailAddressAuthentication`].
impl From<crate::types::EmailAddressAuthenticationAppleId> for EmailAddressAuthentication {
    fn from(val: crate::types::EmailAddressAuthenticationAppleId) -> Self {
        Self::AppleId(Box::new(val))
    }
}

/// Converts a [`crate::types::EmailAddressAuthenticationGoogleId`] into [`EmailAddressAuthentication`].
impl From<crate::types::EmailAddressAuthenticationGoogleId> for EmailAddressAuthentication {
    fn from(val: crate::types::EmailAddressAuthenticationGoogleId) -> Self {
        Self::GoogleId(Box::new(val))
    }
}

/// Describes reset state of an email address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EmailAddressResetState {
    /// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
    #[serde(rename(serialize = "emailAddressResetStateAvailable", deserialize = "emailAddressResetStateAvailable"))]
    Available(Box<crate::types::EmailAddressResetStateAvailable>),
    /// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
    #[serde(rename(serialize = "emailAddressResetStatePending", deserialize = "emailAddressResetStatePending"))]
    Pending(Box<crate::types::EmailAddressResetStatePending>),
}

impl EmailAddressResetState {
    /// Convenience constructor to create a [`EmailAddressResetState::Available`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available(val: crate::types::EmailAddressResetStateAvailable) -> Self {
        Self::Available(Box::new(val))
    }

    /// Convenience constructor to create a [`EmailAddressResetState::Pending`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending(val: crate::types::EmailAddressResetStatePending) -> Self {
        Self::Pending(Box::new(val))
    }

}

/// Converts a [`crate::types::EmailAddressResetStateAvailable`] into [`EmailAddressResetState`].
impl From<crate::types::EmailAddressResetStateAvailable> for EmailAddressResetState {
    fn from(val: crate::types::EmailAddressResetStateAvailable) -> Self {
        Self::Available(Box::new(val))
    }
}

/// Converts a [`crate::types::EmailAddressResetStatePending`] into [`EmailAddressResetState`].
impl From<crate::types::EmailAddressResetStatePending> for EmailAddressResetState {
    fn from(val: crate::types::EmailAddressResetStatePending) -> Self {
        Self::Pending(Box::new(val))
    }
}

/// TDLib `DiffEntity` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiffEntity {
    /// Represents a change of a text
    #[serde(rename(serialize = "diffEntity", deserialize = "diffEntity"))]
    DiffEntity(Box<crate::types::DiffEntity>),
}

impl DiffEntity {
    /// Convenience constructor to create a [`DiffEntity::DiffEntity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn diff_entity(val: crate::types::DiffEntity) -> Self {
        Self::DiffEntity(Box::new(val))
    }

}

/// Converts a [`crate::types::DiffEntity`] into [`DiffEntity`].
impl From<crate::types::DiffEntity> for DiffEntity {
    fn from(val: crate::types::DiffEntity) -> Self {
        Self::DiffEntity(Box::new(val))
    }
}

/// TDLib `TermsOfService` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TermsOfService {
    /// Contains Telegram terms of service
    #[serde(rename(serialize = "termsOfService", deserialize = "termsOfService"))]
    TermsOfService(Box<crate::types::TermsOfService>),
}

impl TermsOfService {
    /// Convenience constructor to create a [`TermsOfService::TermsOfService`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn terms_of_service(val: crate::types::TermsOfService) -> Self {
        Self::TermsOfService(Box::new(val))
    }

}

/// Converts a [`crate::types::TermsOfService`] into [`TermsOfService`].
impl From<crate::types::TermsOfService> for TermsOfService {
    fn from(val: crate::types::TermsOfService) -> Self {
        Self::TermsOfService(Box::new(val))
    }
}

/// TDLib `Passkey` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Passkey {
    /// Describes a passkey
    #[serde(rename(serialize = "passkey", deserialize = "passkey"))]
    Passkey(Box<crate::types::Passkey>),
}

impl Passkey {
    /// Convenience constructor to create a [`Passkey::Passkey`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passkey(val: crate::types::Passkey) -> Self {
        Self::Passkey(Box::new(val))
    }

}

/// Converts a [`crate::types::Passkey`] into [`Passkey`].
impl From<crate::types::Passkey> for Passkey {
    fn from(val: crate::types::Passkey) -> Self {
        Self::Passkey(Box::new(val))
    }
}

/// TDLib `Passkeys` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Passkeys {
    /// Contains a list of passkeys
    #[serde(rename(serialize = "passkeys", deserialize = "passkeys"))]
    Passkeys(Box<crate::types::Passkeys>),
}

impl Passkeys {
    /// Convenience constructor to create a [`Passkeys::Passkeys`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passkeys(val: crate::types::Passkeys) -> Self {
        Self::Passkeys(Box::new(val))
    }

}

/// Converts a [`crate::types::Passkeys`] into [`Passkeys`].
impl From<crate::types::Passkeys> for Passkeys {
    fn from(val: crate::types::Passkeys) -> Self {
        Self::Passkeys(Box::new(val))
    }
}

/// Represents the current authorization state of the TDLib client
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthorizationState {
    /// Initialization parameters are needed. Call setTdlibParameters to provide them
    #[serde(rename(serialize = "authorizationStateWaitTdlibParameters", deserialize = "authorizationStateWaitTdlibParameters"))]
    WaitTdlibParameters,
    /// TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number,
    /// or use requestQrCodeAuthentication, getAuthenticationPasskeyParameters, checkAuthenticationWebToken, or checkAuthenticationBotToken for other authentication options
    #[serde(rename(serialize = "authorizationStateWaitPhoneNumber", deserialize = "authorizationStateWaitPhoneNumber"))]
    WaitPhoneNumber,
    /// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
    #[serde(rename(serialize = "authorizationStateWaitPremiumPurchase", deserialize = "authorizationStateWaitPremiumPurchase"))]
    WaitPremiumPurchase(Box<crate::types::AuthorizationStateWaitPremiumPurchase>),
    /// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
    #[serde(rename(serialize = "authorizationStateWaitEmailAddress", deserialize = "authorizationStateWaitEmailAddress"))]
    WaitEmailAddress(Box<crate::types::AuthorizationStateWaitEmailAddress>),
    /// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
    #[serde(rename(serialize = "authorizationStateWaitEmailCode", deserialize = "authorizationStateWaitEmailCode"))]
    WaitEmailCode(Box<crate::types::AuthorizationStateWaitEmailCode>),
    /// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
    #[serde(rename(serialize = "authorizationStateWaitCode", deserialize = "authorizationStateWaitCode"))]
    WaitCode(Box<crate::types::AuthorizationStateWaitCode>),
    /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
    #[serde(rename(serialize = "authorizationStateWaitOtherDeviceConfirmation", deserialize = "authorizationStateWaitOtherDeviceConfirmation"))]
    WaitOtherDeviceConfirmation(Box<crate::types::AuthorizationStateWaitOtherDeviceConfirmation>),
    /// The user is unregistered and needs to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
    #[serde(rename(serialize = "authorizationStateWaitRegistration", deserialize = "authorizationStateWaitRegistration"))]
    WaitRegistration(Box<crate::types::AuthorizationStateWaitRegistration>),
    /// The user has been authorized, but needs to enter a 2-step verification password to start using the application.
    /// Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
    #[serde(rename(serialize = "authorizationStateWaitPassword", deserialize = "authorizationStateWaitPassword"))]
    WaitPassword(Box<crate::types::AuthorizationStateWaitPassword>),
    /// The user has been successfully authorized. TDLib is now ready to answer general requests
    #[serde(rename(serialize = "authorizationStateReady", deserialize = "authorizationStateReady"))]
    Ready,
    /// The user is currently logging out
    #[serde(rename(serialize = "authorizationStateLoggingOut", deserialize = "authorizationStateLoggingOut"))]
    LoggingOut,
    /// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
    #[serde(rename(serialize = "authorizationStateClosing", deserialize = "authorizationStateClosing"))]
    Closing,
    /// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to
    /// with error code 500. To continue working, one must create a new instance of the TDLib client
    #[serde(rename(serialize = "authorizationStateClosed", deserialize = "authorizationStateClosed"))]
    Closed,
}

impl AuthorizationState {
    /// Convenience constructor to create a [`AuthorizationState::WaitPremiumPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_premium_purchase(val: crate::types::AuthorizationStateWaitPremiumPurchase) -> Self {
        Self::WaitPremiumPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitEmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_email_address(val: crate::types::AuthorizationStateWaitEmailAddress) -> Self {
        Self::WaitEmailAddress(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitEmailCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_email_code(val: crate::types::AuthorizationStateWaitEmailCode) -> Self {
        Self::WaitEmailCode(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_code(val: crate::types::AuthorizationStateWaitCode) -> Self {
        Self::WaitCode(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitOtherDeviceConfirmation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_other_device_confirmation(val: crate::types::AuthorizationStateWaitOtherDeviceConfirmation) -> Self {
        Self::WaitOtherDeviceConfirmation(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitRegistration`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_registration(val: crate::types::AuthorizationStateWaitRegistration) -> Self {
        Self::WaitRegistration(Box::new(val))
    }

    /// Convenience constructor to create a [`AuthorizationState::WaitPassword`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wait_password(val: crate::types::AuthorizationStateWaitPassword) -> Self {
        Self::WaitPassword(Box::new(val))
    }

}

/// Converts a [`crate::types::AuthorizationStateWaitPremiumPurchase`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitPremiumPurchase> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitPremiumPurchase) -> Self {
        Self::WaitPremiumPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitEmailAddress`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitEmailAddress> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitEmailAddress) -> Self {
        Self::WaitEmailAddress(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitEmailCode`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitEmailCode> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitEmailCode) -> Self {
        Self::WaitEmailCode(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitCode`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitCode> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitCode) -> Self {
        Self::WaitCode(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitOtherDeviceConfirmation`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitOtherDeviceConfirmation> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitOtherDeviceConfirmation) -> Self {
        Self::WaitOtherDeviceConfirmation(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitRegistration`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitRegistration> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitRegistration) -> Self {
        Self::WaitRegistration(Box::new(val))
    }
}

/// Converts a [`crate::types::AuthorizationStateWaitPassword`] into [`AuthorizationState`].
impl From<crate::types::AuthorizationStateWaitPassword> for AuthorizationState {
    fn from(val: crate::types::AuthorizationStateWaitPassword) -> Self {
        Self::WaitPassword(Box::new(val))
    }
}

/// Describes parameters to be used for device verification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FirebaseDeviceVerificationParameters {
    /// Device verification must be performed with the SafetyNet Attestation API
    #[serde(rename(serialize = "firebaseDeviceVerificationParametersSafetyNet", deserialize = "firebaseDeviceVerificationParametersSafetyNet"))]
    SafetyNet(Box<crate::types::FirebaseDeviceVerificationParametersSafetyNet>),
    /// Device verification must be performed with the classic Play Integrity verification (https:developer.android.com/google/play/integrity/classic)
    #[serde(rename(serialize = "firebaseDeviceVerificationParametersPlayIntegrity", deserialize = "firebaseDeviceVerificationParametersPlayIntegrity"))]
    PlayIntegrity(Box<crate::types::FirebaseDeviceVerificationParametersPlayIntegrity>),
}

impl FirebaseDeviceVerificationParameters {
    /// Convenience constructor to create a [`FirebaseDeviceVerificationParameters::SafetyNet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn safety_net(val: crate::types::FirebaseDeviceVerificationParametersSafetyNet) -> Self {
        Self::SafetyNet(Box::new(val))
    }

    /// Convenience constructor to create a [`FirebaseDeviceVerificationParameters::PlayIntegrity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn play_integrity(val: crate::types::FirebaseDeviceVerificationParametersPlayIntegrity) -> Self {
        Self::PlayIntegrity(Box::new(val))
    }

}

/// Converts a [`crate::types::FirebaseDeviceVerificationParametersSafetyNet`] into [`FirebaseDeviceVerificationParameters`].
impl From<crate::types::FirebaseDeviceVerificationParametersSafetyNet> for FirebaseDeviceVerificationParameters {
    fn from(val: crate::types::FirebaseDeviceVerificationParametersSafetyNet) -> Self {
        Self::SafetyNet(Box::new(val))
    }
}

/// Converts a [`crate::types::FirebaseDeviceVerificationParametersPlayIntegrity`] into [`FirebaseDeviceVerificationParameters`].
impl From<crate::types::FirebaseDeviceVerificationParametersPlayIntegrity> for FirebaseDeviceVerificationParameters {
    fn from(val: crate::types::FirebaseDeviceVerificationParametersPlayIntegrity) -> Self {
        Self::PlayIntegrity(Box::new(val))
    }
}

/// TDLib `PasswordState` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PasswordState {
    /// Represents the current state of 2-step verification
    #[serde(rename(serialize = "passwordState", deserialize = "passwordState"))]
    PasswordState(Box<crate::types::PasswordState>),
}

impl PasswordState {
    /// Convenience constructor to create a [`PasswordState::PasswordState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn password_state(val: crate::types::PasswordState) -> Self {
        Self::PasswordState(Box::new(val))
    }

}

/// Converts a [`crate::types::PasswordState`] into [`PasswordState`].
impl From<crate::types::PasswordState> for PasswordState {
    fn from(val: crate::types::PasswordState) -> Self {
        Self::PasswordState(Box::new(val))
    }
}

/// TDLib `RecoveryEmailAddress` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RecoveryEmailAddress {
    /// Contains information about the current recovery email address
    #[serde(rename(serialize = "recoveryEmailAddress", deserialize = "recoveryEmailAddress"))]
    RecoveryEmailAddress(Box<crate::types::RecoveryEmailAddress>),
}

impl RecoveryEmailAddress {
    /// Convenience constructor to create a [`RecoveryEmailAddress::RecoveryEmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recovery_email_address(val: crate::types::RecoveryEmailAddress) -> Self {
        Self::RecoveryEmailAddress(Box::new(val))
    }

}

/// Converts a [`crate::types::RecoveryEmailAddress`] into [`RecoveryEmailAddress`].
impl From<crate::types::RecoveryEmailAddress> for RecoveryEmailAddress {
    fn from(val: crate::types::RecoveryEmailAddress) -> Self {
        Self::RecoveryEmailAddress(Box::new(val))
    }
}

/// TDLib `TemporaryPasswordState` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TemporaryPasswordState {
    /// Returns information about the availability of a temporary password, which can be used for payments
    #[serde(rename(serialize = "temporaryPasswordState", deserialize = "temporaryPasswordState"))]
    TemporaryPasswordState(Box<crate::types::TemporaryPasswordState>),
}

impl TemporaryPasswordState {
    /// Convenience constructor to create a [`TemporaryPasswordState::TemporaryPasswordState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn temporary_password_state(val: crate::types::TemporaryPasswordState) -> Self {
        Self::TemporaryPasswordState(Box::new(val))
    }

}

/// Converts a [`crate::types::TemporaryPasswordState`] into [`TemporaryPasswordState`].
impl From<crate::types::TemporaryPasswordState> for TemporaryPasswordState {
    fn from(val: crate::types::TemporaryPasswordState) -> Self {
        Self::TemporaryPasswordState(Box::new(val))
    }
}

/// Part of the face, relative to which a mask is placed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MaskPoint {
    /// The mask is placed relatively to the forehead
    #[serde(rename(serialize = "maskPointForehead", deserialize = "maskPointForehead"))]
    Forehead,
    /// The mask is placed relatively to the eyes
    #[serde(rename(serialize = "maskPointEyes", deserialize = "maskPointEyes"))]
    Eyes,
    /// The mask is placed relatively to the mouth
    #[serde(rename(serialize = "maskPointMouth", deserialize = "maskPointMouth"))]
    Mouth,
    /// The mask is placed relatively to the chin
    #[serde(rename(serialize = "maskPointChin", deserialize = "maskPointChin"))]
    Chin,
}

/// TDLib `MaskPosition` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MaskPosition {
    /// Position on a photo where a mask is placed
    #[serde(rename(serialize = "maskPosition", deserialize = "maskPosition"))]
    MaskPosition(Box<crate::types::MaskPosition>),
}

impl MaskPosition {
    /// Convenience constructor to create a [`MaskPosition::MaskPosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mask_position(val: crate::types::MaskPosition) -> Self {
        Self::MaskPosition(Box::new(val))
    }

}

/// Converts a [`crate::types::MaskPosition`] into [`MaskPosition`].
impl From<crate::types::MaskPosition> for MaskPosition {
    fn from(val: crate::types::MaskPosition) -> Self {
        Self::MaskPosition(Box::new(val))
    }
}

/// TDLib `ClosedVectorPath` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ClosedVectorPath {
    /// Represents a closed vector path. The path begins at the end point of the last command. The coordinate system origin is in the upper-left corner
    #[serde(rename(serialize = "closedVectorPath", deserialize = "closedVectorPath"))]
    ClosedVectorPath(Box<crate::types::ClosedVectorPath>),
}

impl ClosedVectorPath {
    /// Convenience constructor to create a [`ClosedVectorPath::ClosedVectorPath`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn closed_vector_path(val: crate::types::ClosedVectorPath) -> Self {
        Self::ClosedVectorPath(Box::new(val))
    }

}

/// Converts a [`crate::types::ClosedVectorPath`] into [`ClosedVectorPath`].
impl From<crate::types::ClosedVectorPath> for ClosedVectorPath {
    fn from(val: crate::types::ClosedVectorPath) -> Self {
        Self::ClosedVectorPath(Box::new(val))
    }
}

/// TDLib `Outline` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Outline {
    /// Represents outline of an image
    #[serde(rename(serialize = "outline", deserialize = "outline"))]
    Outline(Box<crate::types::Outline>),
}

impl Outline {
    /// Convenience constructor to create a [`Outline::Outline`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn outline(val: crate::types::Outline) -> Self {
        Self::Outline(Box::new(val))
    }

}

/// Converts a [`crate::types::Outline`] into [`Outline`].
impl From<crate::types::Outline> for Outline {
    fn from(val: crate::types::Outline) -> Self {
        Self::Outline(Box::new(val))
    }
}

/// TDLib `ChecklistTask` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChecklistTask {
    /// Describes a task in a checklist
    #[serde(rename(serialize = "checklistTask", deserialize = "checklistTask"))]
    ChecklistTask(Box<crate::types::ChecklistTask>),
}

impl ChecklistTask {
    /// Convenience constructor to create a [`ChecklistTask::ChecklistTask`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn checklist_task(val: crate::types::ChecklistTask) -> Self {
        Self::ChecklistTask(Box::new(val))
    }

}

/// Converts a [`crate::types::ChecklistTask`] into [`ChecklistTask`].
impl From<crate::types::ChecklistTask> for ChecklistTask {
    fn from(val: crate::types::ChecklistTask) -> Self {
        Self::ChecklistTask(Box::new(val))
    }
}

/// TDLib `InputChecklistTask` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChecklistTask {
    /// Describes a task in a checklist to be sent
    #[serde(rename(serialize = "inputChecklistTask", deserialize = "inputChecklistTask"))]
    InputChecklistTask(Box<crate::types::InputChecklistTask>),
}

impl InputChecklistTask {
    /// Convenience constructor to create a [`InputChecklistTask::InputChecklistTask`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_checklist_task(val: crate::types::InputChecklistTask) -> Self {
        Self::InputChecklistTask(Box::new(val))
    }

}

/// Converts a [`crate::types::InputChecklistTask`] into [`InputChecklistTask`].
impl From<crate::types::InputChecklistTask> for InputChecklistTask {
    fn from(val: crate::types::InputChecklistTask) -> Self {
        Self::InputChecklistTask(Box::new(val))
    }
}

/// TDLib `Checklist` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Checklist {
    /// Describes a checklist
    #[serde(rename(serialize = "checklist", deserialize = "checklist"))]
    Checklist(Box<crate::types::Checklist>),
}

impl Checklist {
    /// Convenience constructor to create a [`Checklist::Checklist`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn checklist(val: crate::types::Checklist) -> Self {
        Self::Checklist(Box::new(val))
    }

}

/// Converts a [`crate::types::Checklist`] into [`Checklist`].
impl From<crate::types::Checklist> for Checklist {
    fn from(val: crate::types::Checklist) -> Self {
        Self::Checklist(Box::new(val))
    }
}

/// TDLib `InputChecklist` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChecklist {
    /// Describes a checklist to be sent
    #[serde(rename(serialize = "inputChecklist", deserialize = "inputChecklist"))]
    InputChecklist(Box<crate::types::InputChecklist>),
}

impl InputChecklist {
    /// Convenience constructor to create a [`InputChecklist::InputChecklist`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_checklist(val: crate::types::InputChecklist) -> Self {
        Self::InputChecklist(Box::new(val))
    }

}

/// Converts a [`crate::types::InputChecklist`] into [`InputChecklist`].
impl From<crate::types::InputChecklist> for InputChecklist {
    fn from(val: crate::types::InputChecklist) -> Self {
        Self::InputChecklist(Box::new(val))
    }
}

/// TDLib `VoiceNote` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VoiceNote {
    /// Describes a voice note
    #[serde(rename(serialize = "voiceNote", deserialize = "voiceNote"))]
    VoiceNote(Box<crate::types::VoiceNote>),
}

impl VoiceNote {
    /// Convenience constructor to create a [`VoiceNote::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::VoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

}

/// Converts a [`crate::types::VoiceNote`] into [`VoiceNote`].
impl From<crate::types::VoiceNote> for VoiceNote {
    fn from(val: crate::types::VoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// TDLib `Location` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Location {
    /// Describes a location on planet Earth
    #[serde(rename(serialize = "location", deserialize = "location"))]
    Location(Box<crate::types::Location>),
}

impl Location {
    /// Convenience constructor to create a [`Location::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::Location) -> Self {
        Self::Location(Box::new(val))
    }

}

/// Converts a [`crate::types::Location`] into [`Location`].
impl From<crate::types::Location> for Location {
    fn from(val: crate::types::Location) -> Self {
        Self::Location(Box::new(val))
    }
}

/// TDLib `LiveLocation` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LiveLocation {
    /// A live location
    #[serde(rename(serialize = "liveLocation", deserialize = "liveLocation"))]
    LiveLocation(Box<crate::types::LiveLocation>),
}

impl LiveLocation {
    /// Convenience constructor to create a [`LiveLocation::LiveLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_location(val: crate::types::LiveLocation) -> Self {
        Self::LiveLocation(Box::new(val))
    }

}

/// Converts a [`crate::types::LiveLocation`] into [`LiveLocation`].
impl From<crate::types::LiveLocation> for LiveLocation {
    fn from(val: crate::types::LiveLocation) -> Self {
        Self::LiveLocation(Box::new(val))
    }
}

/// TDLib `Venue` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Venue {
    /// Describes a venue
    #[serde(rename(serialize = "venue", deserialize = "venue"))]
    Venue(Box<crate::types::Venue>),
}

impl Venue {
    /// Convenience constructor to create a [`Venue::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::Venue) -> Self {
        Self::Venue(Box::new(val))
    }

}

/// Converts a [`crate::types::Venue`] into [`Venue`].
impl From<crate::types::Venue> for Venue {
    fn from(val: crate::types::Venue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// TDLib `Game` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Game {
    /// Describes a game. Use getInternalLink with internalLinkTypeGame to share the game
    #[serde(rename(serialize = "game", deserialize = "game"))]
    Game(Box<crate::types::Game>),
}

impl Game {
    /// Convenience constructor to create a [`Game::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::Game) -> Self {
        Self::Game(Box::new(val))
    }

}

/// Converts a [`crate::types::Game`] into [`Game`].
impl From<crate::types::Game> for Game {
    fn from(val: crate::types::Game) -> Self {
        Self::Game(Box::new(val))
    }
}

/// TDLib `WebApp` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebApp {
    /// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
    #[serde(rename(serialize = "webApp", deserialize = "webApp"))]
    WebApp(Box<crate::types::WebApp>),
}

impl WebApp {
    /// Convenience constructor to create a [`WebApp::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::WebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::WebApp`] into [`WebApp`].
impl From<crate::types::WebApp> for WebApp {
    fn from(val: crate::types::WebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// TDLib `Background` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Background {
    /// Describes a chat background
    #[serde(rename(serialize = "background", deserialize = "background"))]
    Background(Box<crate::types::Background>),
}

impl Background {
    /// Convenience constructor to create a [`Background::Background`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn background(val: crate::types::Background) -> Self {
        Self::Background(Box::new(val))
    }

}

/// Converts a [`crate::types::Background`] into [`Background`].
impl From<crate::types::Background> for Background {
    fn from(val: crate::types::Background) -> Self {
        Self::Background(Box::new(val))
    }
}

/// TDLib `Backgrounds` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Backgrounds {
    /// Contains a list of backgrounds
    #[serde(rename(serialize = "backgrounds", deserialize = "backgrounds"))]
    Backgrounds(Box<crate::types::Backgrounds>),
}

impl Backgrounds {
    /// Convenience constructor to create a [`Backgrounds::Backgrounds`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn backgrounds(val: crate::types::Backgrounds) -> Self {
        Self::Backgrounds(Box::new(val))
    }

}

/// Converts a [`crate::types::Backgrounds`] into [`Backgrounds`].
impl From<crate::types::Backgrounds> for Backgrounds {
    fn from(val: crate::types::Backgrounds) -> Self {
        Self::Backgrounds(Box::new(val))
    }
}

/// TDLib `VerificationStatus` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VerificationStatus {
    /// Contains information about verification status of a chat or a user
    #[serde(rename(serialize = "verificationStatus", deserialize = "verificationStatus"))]
    VerificationStatus(Box<crate::types::VerificationStatus>),
}

impl VerificationStatus {
    /// Convenience constructor to create a [`VerificationStatus::VerificationStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn verification_status(val: crate::types::VerificationStatus) -> Self {
        Self::VerificationStatus(Box::new(val))
    }

}

/// Converts a [`crate::types::VerificationStatus`] into [`VerificationStatus`].
impl From<crate::types::VerificationStatus> for VerificationStatus {
    fn from(val: crate::types::VerificationStatus) -> Self {
        Self::VerificationStatus(Box::new(val))
    }
}

/// TDLib `Birthdate` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Birthdate {
    /// Represents a birthdate of a user
    #[serde(rename(serialize = "birthdate", deserialize = "birthdate"))]
    Birthdate(Box<crate::types::Birthdate>),
}

impl Birthdate {
    /// Convenience constructor to create a [`Birthdate::Birthdate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn birthdate(val: crate::types::Birthdate) -> Self {
        Self::Birthdate(Box::new(val))
    }

}

/// Converts a [`crate::types::Birthdate`] into [`Birthdate`].
impl From<crate::types::Birthdate> for Birthdate {
    fn from(val: crate::types::Birthdate) -> Self {
        Self::Birthdate(Box::new(val))
    }
}

/// TDLib `ThemeParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThemeParameters {
    /// Contains parameters of the application theme
    #[serde(rename(serialize = "themeParameters", deserialize = "themeParameters"))]
    ThemeParameters(Box<crate::types::ThemeParameters>),
}

impl ThemeParameters {
    /// Convenience constructor to create a [`ThemeParameters::ThemeParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn theme_parameters(val: crate::types::ThemeParameters) -> Self {
        Self::ThemeParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::ThemeParameters`] into [`ThemeParameters`].
impl From<crate::types::ThemeParameters> for ThemeParameters {
    fn from(val: crate::types::ThemeParameters) -> Self {
        Self::ThemeParameters(Box::new(val))
    }
}

/// Describes mode in which a Web App is opened
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppOpenMode {
    /// The Web App is opened in the compact mode
    #[serde(rename(serialize = "webAppOpenModeCompact", deserialize = "webAppOpenModeCompact"))]
    Compact,
    /// The Web App is opened in the full-size mode
    #[serde(rename(serialize = "webAppOpenModeFullSize", deserialize = "webAppOpenModeFullSize"))]
    FullSize,
    /// The Web App is opened in the full-screen mode
    #[serde(rename(serialize = "webAppOpenModeFullScreen", deserialize = "webAppOpenModeFullScreen"))]
    FullScreen,
}

/// TDLib `FoundWebApp` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundWebApp {
    /// Contains information about a Web App found by its short name
    #[serde(rename(serialize = "foundWebApp", deserialize = "foundWebApp"))]
    FoundWebApp(Box<crate::types::FoundWebApp>),
}

impl FoundWebApp {
    /// Convenience constructor to create a [`FoundWebApp::FoundWebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_web_app(val: crate::types::FoundWebApp) -> Self {
        Self::FoundWebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundWebApp`] into [`FoundWebApp`].
impl From<crate::types::FoundWebApp> for FoundWebApp {
    fn from(val: crate::types::FoundWebApp) -> Self {
        Self::FoundWebApp(Box::new(val))
    }
}

/// TDLib `WebAppUrl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppUrl {
    /// Contains information about a Web App URL
    #[serde(rename(serialize = "webAppUrl", deserialize = "webAppUrl"))]
    WebAppUrl(Box<crate::types::WebAppUrl>),
}

impl WebAppUrl {
    /// Convenience constructor to create a [`WebAppUrl::WebAppUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app_url(val: crate::types::WebAppUrl) -> Self {
        Self::WebAppUrl(Box::new(val))
    }

}

/// Converts a [`crate::types::WebAppUrl`] into [`WebAppUrl`].
impl From<crate::types::WebAppUrl> for WebAppUrl {
    fn from(val: crate::types::WebAppUrl) -> Self {
        Self::WebAppUrl(Box::new(val))
    }
}

/// TDLib `WebAppInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppInfo {
    /// Contains information about a Web App
    #[serde(rename(serialize = "webAppInfo", deserialize = "webAppInfo"))]
    WebAppInfo(Box<crate::types::WebAppInfo>),
}

impl WebAppInfo {
    /// Convenience constructor to create a [`WebAppInfo::WebAppInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app_info(val: crate::types::WebAppInfo) -> Self {
        Self::WebAppInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::WebAppInfo`] into [`WebAppInfo`].
impl From<crate::types::WebAppInfo> for WebAppInfo {
    fn from(val: crate::types::WebAppInfo) -> Self {
        Self::WebAppInfo(Box::new(val))
    }
}

/// TDLib `MainWebApp` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MainWebApp {
    /// Contains information about the main Web App of a bot
    #[serde(rename(serialize = "mainWebApp", deserialize = "mainWebApp"))]
    MainWebApp(Box<crate::types::MainWebApp>),
}

impl MainWebApp {
    /// Convenience constructor to create a [`MainWebApp::MainWebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn main_web_app(val: crate::types::MainWebApp) -> Self {
        Self::MainWebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::MainWebApp`] into [`MainWebApp`].
impl From<crate::types::MainWebApp> for MainWebApp {
    fn from(val: crate::types::MainWebApp) -> Self {
        Self::MainWebApp(Box::new(val))
    }
}

/// TDLib `WebAppOpenParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppOpenParameters {
    /// Options to be used when a Web App is opened
    #[serde(rename(serialize = "webAppOpenParameters", deserialize = "webAppOpenParameters"))]
    WebAppOpenParameters(Box<crate::types::WebAppOpenParameters>),
}

impl WebAppOpenParameters {
    /// Convenience constructor to create a [`WebAppOpenParameters::WebAppOpenParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app_open_parameters(val: crate::types::WebAppOpenParameters) -> Self {
        Self::WebAppOpenParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::WebAppOpenParameters`] into [`WebAppOpenParameters`].
impl From<crate::types::WebAppOpenParameters> for WebAppOpenParameters {
    fn from(val: crate::types::WebAppOpenParameters) -> Self {
        Self::WebAppOpenParameters(Box::new(val))
    }
}

/// Describes price of a resold gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResalePrice {
    /// Describes price of a resold gift in Telegram Stars
    #[serde(rename(serialize = "giftResalePriceStar", deserialize = "giftResalePriceStar"))]
    Star(Box<crate::types::GiftResalePriceStar>),
    /// Describes price of a resold gift in TON Grams
    #[serde(rename(serialize = "giftResalePriceGram", deserialize = "giftResalePriceGram"))]
    Gram(Box<crate::types::GiftResalePriceGram>),
}

impl GiftResalePrice {
    /// Convenience constructor to create a [`GiftResalePrice::Star`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star(val: crate::types::GiftResalePriceStar) -> Self {
        Self::Star(Box::new(val))
    }

    /// Convenience constructor to create a [`GiftResalePrice::Gram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gram(val: crate::types::GiftResalePriceGram) -> Self {
        Self::Gram(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftResalePriceStar`] into [`GiftResalePrice`].
impl From<crate::types::GiftResalePriceStar> for GiftResalePrice {
    fn from(val: crate::types::GiftResalePriceStar) -> Self {
        Self::Star(Box::new(val))
    }
}

/// Converts a [`crate::types::GiftResalePriceGram`] into [`GiftResalePrice`].
impl From<crate::types::GiftResalePriceGram> for GiftResalePrice {
    fn from(val: crate::types::GiftResalePriceGram) -> Self {
        Self::Gram(Box::new(val))
    }
}

/// Describes state of a gift purchase offer
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftPurchaseOfferState {
    /// The offer must be accepted or rejected
    #[serde(rename(serialize = "giftPurchaseOfferStatePending", deserialize = "giftPurchaseOfferStatePending"))]
    Pending,
    /// The offer was accepted
    #[serde(rename(serialize = "giftPurchaseOfferStateAccepted", deserialize = "giftPurchaseOfferStateAccepted"))]
    Accepted,
    /// The offer was rejected
    #[serde(rename(serialize = "giftPurchaseOfferStateRejected", deserialize = "giftPurchaseOfferStateRejected"))]
    Rejected,
}

/// Describes price of a suggested post
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostPrice {
    /// Describes price of a suggested post in Telegram Stars
    #[serde(rename(serialize = "suggestedPostPriceStar", deserialize = "suggestedPostPriceStar"))]
    Star(Box<crate::types::SuggestedPostPriceStar>),
    /// Describes price of a suggested post in TON Grams
    #[serde(rename(serialize = "suggestedPostPriceGram", deserialize = "suggestedPostPriceGram"))]
    Gram(Box<crate::types::SuggestedPostPriceGram>),
}

impl SuggestedPostPrice {
    /// Convenience constructor to create a [`SuggestedPostPrice::Star`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star(val: crate::types::SuggestedPostPriceStar) -> Self {
        Self::Star(Box::new(val))
    }

    /// Convenience constructor to create a [`SuggestedPostPrice::Gram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gram(val: crate::types::SuggestedPostPriceGram) -> Self {
        Self::Gram(Box::new(val))
    }

}

/// Converts a [`crate::types::SuggestedPostPriceStar`] into [`SuggestedPostPrice`].
impl From<crate::types::SuggestedPostPriceStar> for SuggestedPostPrice {
    fn from(val: crate::types::SuggestedPostPriceStar) -> Self {
        Self::Star(Box::new(val))
    }
}

/// Converts a [`crate::types::SuggestedPostPriceGram`] into [`SuggestedPostPrice`].
impl From<crate::types::SuggestedPostPriceGram> for SuggestedPostPrice {
    fn from(val: crate::types::SuggestedPostPriceGram) -> Self {
        Self::Gram(Box::new(val))
    }
}

/// Describes state of a suggested post
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostState {
    /// The post must be approved or declined
    #[serde(rename(serialize = "suggestedPostStatePending", deserialize = "suggestedPostStatePending"))]
    Pending,
    /// The post was approved
    #[serde(rename(serialize = "suggestedPostStateApproved", deserialize = "suggestedPostStateApproved"))]
    Approved,
    /// The post was declined
    #[serde(rename(serialize = "suggestedPostStateDeclined", deserialize = "suggestedPostStateDeclined"))]
    Declined,
}

/// TDLib `SuggestedPostInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostInfo {
    /// Contains information about a suggested post. If the post can be approved or declined, then changes to the post can be also suggested. Use sendMessage with reply to the message
    /// and suggested post information to suggest message changes. Use addOffer to suggest price or time changes
    #[serde(rename(serialize = "suggestedPostInfo", deserialize = "suggestedPostInfo"))]
    SuggestedPostInfo(Box<crate::types::SuggestedPostInfo>),
}

impl SuggestedPostInfo {
    /// Convenience constructor to create a [`SuggestedPostInfo::SuggestedPostInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_post_info(val: crate::types::SuggestedPostInfo) -> Self {
        Self::SuggestedPostInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::SuggestedPostInfo`] into [`SuggestedPostInfo`].
impl From<crate::types::SuggestedPostInfo> for SuggestedPostInfo {
    fn from(val: crate::types::SuggestedPostInfo) -> Self {
        Self::SuggestedPostInfo(Box::new(val))
    }
}

/// TDLib `InputSuggestedPostInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputSuggestedPostInfo {
    /// Contains information about a post to suggest
    #[serde(rename(serialize = "inputSuggestedPostInfo", deserialize = "inputSuggestedPostInfo"))]
    InputSuggestedPostInfo(Box<crate::types::InputSuggestedPostInfo>),
}

impl InputSuggestedPostInfo {
    /// Convenience constructor to create a [`InputSuggestedPostInfo::InputSuggestedPostInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_suggested_post_info(val: crate::types::InputSuggestedPostInfo) -> Self {
        Self::InputSuggestedPostInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::InputSuggestedPostInfo`] into [`InputSuggestedPostInfo`].
impl From<crate::types::InputSuggestedPostInfo> for InputSuggestedPostInfo {
    fn from(val: crate::types::InputSuggestedPostInfo) -> Self {
        Self::InputSuggestedPostInfo(Box::new(val))
    }
}

/// Describes reason for refund of the payment for a suggested post
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostRefundReason {
    /// The post was refunded, because it was deleted by channel administrators in less than getOption("suggested_post_lifetime_min") seconds
    #[serde(rename(serialize = "suggestedPostRefundReasonPostDeleted", deserialize = "suggestedPostRefundReasonPostDeleted"))]
    PostDeleted,
    /// The post was refunded, because the payment for the post was refunded
    #[serde(rename(serialize = "suggestedPostRefundReasonPaymentRefunded", deserialize = "suggestedPostRefundReasonPaymentRefunded"))]
    PaymentRefunded,
}

/// TDLib `StarAmount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarAmount {
    /// Describes a possibly non-integer Telegram Star amount
    #[serde(rename(serialize = "starAmount", deserialize = "starAmount"))]
    StarAmount(Box<crate::types::StarAmount>),
}

impl StarAmount {
    /// Convenience constructor to create a [`StarAmount::StarAmount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_amount(val: crate::types::StarAmount) -> Self {
        Self::StarAmount(Box::new(val))
    }

}

/// Converts a [`crate::types::StarAmount`] into [`StarAmount`].
impl From<crate::types::StarAmount> for StarAmount {
    fn from(val: crate::types::StarAmount) -> Self {
        Self::StarAmount(Box::new(val))
    }
}

/// TDLib `ProductInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProductInfo {
    /// Contains information about a product that can be paid with invoice
    #[serde(rename(serialize = "productInfo", deserialize = "productInfo"))]
    ProductInfo(Box<crate::types::ProductInfo>),
}

impl ProductInfo {
    /// Convenience constructor to create a [`ProductInfo::ProductInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn product_info(val: crate::types::ProductInfo) -> Self {
        Self::ProductInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ProductInfo`] into [`ProductInfo`].
impl From<crate::types::ProductInfo> for ProductInfo {
    fn from(val: crate::types::ProductInfo) -> Self {
        Self::ProductInfo(Box::new(val))
    }
}

/// TDLib `AcceptedGiftTypes` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AcceptedGiftTypes {
    /// Describes gift types that are accepted by a user
    #[serde(rename(serialize = "acceptedGiftTypes", deserialize = "acceptedGiftTypes"))]
    AcceptedGiftTypes(Box<crate::types::AcceptedGiftTypes>),
}

impl AcceptedGiftTypes {
    /// Convenience constructor to create a [`AcceptedGiftTypes::AcceptedGiftTypes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn accepted_gift_types(val: crate::types::AcceptedGiftTypes) -> Self {
        Self::AcceptedGiftTypes(Box::new(val))
    }

}

/// Converts a [`crate::types::AcceptedGiftTypes`] into [`AcceptedGiftTypes`].
impl From<crate::types::AcceptedGiftTypes> for AcceptedGiftTypes {
    fn from(val: crate::types::AcceptedGiftTypes) -> Self {
        Self::AcceptedGiftTypes(Box::new(val))
    }
}

/// TDLib `GiftSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftSettings {
    /// Contains settings for gift receiving for a user
    #[serde(rename(serialize = "giftSettings", deserialize = "giftSettings"))]
    GiftSettings(Box<crate::types::GiftSettings>),
}

impl GiftSettings {
    /// Convenience constructor to create a [`GiftSettings::GiftSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_settings(val: crate::types::GiftSettings) -> Self {
        Self::GiftSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftSettings`] into [`GiftSettings`].
impl From<crate::types::GiftSettings> for GiftSettings {
    fn from(val: crate::types::GiftSettings) -> Self {
        Self::GiftSettings(Box::new(val))
    }
}

/// TDLib `GiftAuction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuction {
    /// Describes an auction on which a gift can be purchased
    #[serde(rename(serialize = "giftAuction", deserialize = "giftAuction"))]
    GiftAuction(Box<crate::types::GiftAuction>),
}

impl GiftAuction {
    /// Convenience constructor to create a [`GiftAuction::GiftAuction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction(val: crate::types::GiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftAuction`] into [`GiftAuction`].
impl From<crate::types::GiftAuction> for GiftAuction {
    fn from(val: crate::types::GiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }
}

/// TDLib `GiftBackground` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftBackground {
    /// Describes background of a gift
    #[serde(rename(serialize = "giftBackground", deserialize = "giftBackground"))]
    GiftBackground(Box<crate::types::GiftBackground>),
}

impl GiftBackground {
    /// Convenience constructor to create a [`GiftBackground::GiftBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_background(val: crate::types::GiftBackground) -> Self {
        Self::GiftBackground(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftBackground`] into [`GiftBackground`].
impl From<crate::types::GiftBackground> for GiftBackground {
    fn from(val: crate::types::GiftBackground) -> Self {
        Self::GiftBackground(Box::new(val))
    }
}

/// TDLib `GiftPurchaseLimits` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftPurchaseLimits {
    /// Describes the maximum number of times that a specific gift can be purchased
    #[serde(rename(serialize = "giftPurchaseLimits", deserialize = "giftPurchaseLimits"))]
    GiftPurchaseLimits(Box<crate::types::GiftPurchaseLimits>),
}

impl GiftPurchaseLimits {
    /// Convenience constructor to create a [`GiftPurchaseLimits::GiftPurchaseLimits`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_purchase_limits(val: crate::types::GiftPurchaseLimits) -> Self {
        Self::GiftPurchaseLimits(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftPurchaseLimits`] into [`GiftPurchaseLimits`].
impl From<crate::types::GiftPurchaseLimits> for GiftPurchaseLimits {
    fn from(val: crate::types::GiftPurchaseLimits) -> Self {
        Self::GiftPurchaseLimits(Box::new(val))
    }
}

/// TDLib `GiftResaleParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResaleParameters {
    /// Describes parameters of a unique gift available for resale
    #[serde(rename(serialize = "giftResaleParameters", deserialize = "giftResaleParameters"))]
    GiftResaleParameters(Box<crate::types::GiftResaleParameters>),
}

impl GiftResaleParameters {
    /// Convenience constructor to create a [`GiftResaleParameters::GiftResaleParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_resale_parameters(val: crate::types::GiftResaleParameters) -> Self {
        Self::GiftResaleParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftResaleParameters`] into [`GiftResaleParameters`].
impl From<crate::types::GiftResaleParameters> for GiftResaleParameters {
    fn from(val: crate::types::GiftResaleParameters) -> Self {
        Self::GiftResaleParameters(Box::new(val))
    }
}

/// TDLib `GiftCollection` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftCollection {
    /// Describes collection of gifts
    #[serde(rename(serialize = "giftCollection", deserialize = "giftCollection"))]
    GiftCollection(Box<crate::types::GiftCollection>),
}

impl GiftCollection {
    /// Convenience constructor to create a [`GiftCollection::GiftCollection`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_collection(val: crate::types::GiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftCollection`] into [`GiftCollection`].
impl From<crate::types::GiftCollection> for GiftCollection {
    fn from(val: crate::types::GiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }
}

/// TDLib `GiftCollections` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftCollections {
    /// Contains a list of gift collections
    #[serde(rename(serialize = "giftCollections", deserialize = "giftCollections"))]
    GiftCollections(Box<crate::types::GiftCollections>),
}

impl GiftCollections {
    /// Convenience constructor to create a [`GiftCollections::GiftCollections`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_collections(val: crate::types::GiftCollections) -> Self {
        Self::GiftCollections(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftCollections`] into [`GiftCollections`].
impl From<crate::types::GiftCollections> for GiftCollections {
    fn from(val: crate::types::GiftCollections) -> Self {
        Self::GiftCollections(Box::new(val))
    }
}

/// Describes whether a gift can be sent now by the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanSendGiftResult {
    /// The gift can be sent now by the current user
    #[serde(rename(serialize = "canSendGiftResultOk", deserialize = "canSendGiftResultOk"))]
    Ok,
    /// The gift can't be sent now by the current user
    #[serde(rename(serialize = "canSendGiftResultFail", deserialize = "canSendGiftResultFail"))]
    Fail(Box<crate::types::CanSendGiftResultFail>),
}

impl CanSendGiftResult {
    /// Convenience constructor to create a [`CanSendGiftResult::Fail`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fail(val: crate::types::CanSendGiftResultFail) -> Self {
        Self::Fail(Box::new(val))
    }

}

/// Converts a [`crate::types::CanSendGiftResultFail`] into [`CanSendGiftResult`].
impl From<crate::types::CanSendGiftResultFail> for CanSendGiftResult {
    fn from(val: crate::types::CanSendGiftResultFail) -> Self {
        Self::Fail(Box::new(val))
    }
}

/// Describes origin from which the upgraded gift was obtained
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftOrigin {
    /// The gift was obtained by upgrading of a previously received gift
    #[serde(rename(serialize = "upgradedGiftOriginUpgrade", deserialize = "upgradedGiftOriginUpgrade"))]
    Upgrade(Box<crate::types::UpgradedGiftOriginUpgrade>),
    /// The gift was transferred from another owner
    #[serde(rename(serialize = "upgradedGiftOriginTransfer", deserialize = "upgradedGiftOriginTransfer"))]
    Transfer,
    /// The gift was bought from another user
    #[serde(rename(serialize = "upgradedGiftOriginResale", deserialize = "upgradedGiftOriginResale"))]
    Resale(Box<crate::types::UpgradedGiftOriginResale>),
    /// The gift was assigned from blockchain and isn't owned by the current user. The gift can't be transferred, resold or withdrawn to blockchain
    #[serde(rename(serialize = "upgradedGiftOriginBlockchain", deserialize = "upgradedGiftOriginBlockchain"))]
    Blockchain,
    /// The sender or receiver of the message has paid for upgrade of the gift, which has been completed
    #[serde(rename(serialize = "upgradedGiftOriginPrepaidUpgrade", deserialize = "upgradedGiftOriginPrepaidUpgrade"))]
    PrepaidUpgrade,
    /// The gift was bought through an offer
    #[serde(rename(serialize = "upgradedGiftOriginOffer", deserialize = "upgradedGiftOriginOffer"))]
    Offer(Box<crate::types::UpgradedGiftOriginOffer>),
    /// The gift was crafted from other gifts
    #[serde(rename(serialize = "upgradedGiftOriginCraft", deserialize = "upgradedGiftOriginCraft"))]
    Craft,
}

impl UpgradedGiftOrigin {
    /// Convenience constructor to create a [`UpgradedGiftOrigin::Upgrade`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgrade(val: crate::types::UpgradedGiftOriginUpgrade) -> Self {
        Self::Upgrade(Box::new(val))
    }

    /// Convenience constructor to create a [`UpgradedGiftOrigin::Resale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn resale(val: crate::types::UpgradedGiftOriginResale) -> Self {
        Self::Resale(Box::new(val))
    }

    /// Convenience constructor to create a [`UpgradedGiftOrigin::Offer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn offer(val: crate::types::UpgradedGiftOriginOffer) -> Self {
        Self::Offer(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftOriginUpgrade`] into [`UpgradedGiftOrigin`].
impl From<crate::types::UpgradedGiftOriginUpgrade> for UpgradedGiftOrigin {
    fn from(val: crate::types::UpgradedGiftOriginUpgrade) -> Self {
        Self::Upgrade(Box::new(val))
    }
}

/// Converts a [`crate::types::UpgradedGiftOriginResale`] into [`UpgradedGiftOrigin`].
impl From<crate::types::UpgradedGiftOriginResale> for UpgradedGiftOrigin {
    fn from(val: crate::types::UpgradedGiftOriginResale) -> Self {
        Self::Resale(Box::new(val))
    }
}

/// Converts a [`crate::types::UpgradedGiftOriginOffer`] into [`UpgradedGiftOrigin`].
impl From<crate::types::UpgradedGiftOriginOffer> for UpgradedGiftOrigin {
    fn from(val: crate::types::UpgradedGiftOriginOffer) -> Self {
        Self::Offer(Box::new(val))
    }
}

/// Describes rarity of an upgraded gift attribute
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftAttributeRarity {
    /// The rarity is represented as the numeric frequency of the model
    #[serde(rename(serialize = "upgradedGiftAttributeRarityPerMille", deserialize = "upgradedGiftAttributeRarityPerMille"))]
    PerMille(Box<crate::types::UpgradedGiftAttributeRarityPerMille>),
    /// The attribute is uncommon
    #[serde(rename(serialize = "upgradedGiftAttributeRarityUncommon", deserialize = "upgradedGiftAttributeRarityUncommon"))]
    Uncommon,
    /// The attribute is rare
    #[serde(rename(serialize = "upgradedGiftAttributeRarityRare", deserialize = "upgradedGiftAttributeRarityRare"))]
    Rare,
    /// The attribute is epic
    #[serde(rename(serialize = "upgradedGiftAttributeRarityEpic", deserialize = "upgradedGiftAttributeRarityEpic"))]
    Epic,
    /// The attribute is legendary
    #[serde(rename(serialize = "upgradedGiftAttributeRarityLegendary", deserialize = "upgradedGiftAttributeRarityLegendary"))]
    Legendary,
}

impl UpgradedGiftAttributeRarity {
    /// Convenience constructor to create a [`UpgradedGiftAttributeRarity::PerMille`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn per_mille(val: crate::types::UpgradedGiftAttributeRarityPerMille) -> Self {
        Self::PerMille(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftAttributeRarityPerMille`] into [`UpgradedGiftAttributeRarity`].
impl From<crate::types::UpgradedGiftAttributeRarityPerMille> for UpgradedGiftAttributeRarity {
    fn from(val: crate::types::UpgradedGiftAttributeRarityPerMille) -> Self {
        Self::PerMille(Box::new(val))
    }
}

/// TDLib `UpgradedGiftModel` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftModel {
    /// Describes a model of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftModel", deserialize = "upgradedGiftModel"))]
    UpgradedGiftModel(Box<crate::types::UpgradedGiftModel>),
}

impl UpgradedGiftModel {
    /// Convenience constructor to create a [`UpgradedGiftModel::UpgradedGiftModel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_model(val: crate::types::UpgradedGiftModel) -> Self {
        Self::UpgradedGiftModel(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftModel`] into [`UpgradedGiftModel`].
impl From<crate::types::UpgradedGiftModel> for UpgradedGiftModel {
    fn from(val: crate::types::UpgradedGiftModel) -> Self {
        Self::UpgradedGiftModel(Box::new(val))
    }
}

/// TDLib `UpgradedGiftSymbol` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftSymbol {
    /// Describes a symbol shown on the pattern of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftSymbol", deserialize = "upgradedGiftSymbol"))]
    UpgradedGiftSymbol(Box<crate::types::UpgradedGiftSymbol>),
}

impl UpgradedGiftSymbol {
    /// Convenience constructor to create a [`UpgradedGiftSymbol::UpgradedGiftSymbol`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_symbol(val: crate::types::UpgradedGiftSymbol) -> Self {
        Self::UpgradedGiftSymbol(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftSymbol`] into [`UpgradedGiftSymbol`].
impl From<crate::types::UpgradedGiftSymbol> for UpgradedGiftSymbol {
    fn from(val: crate::types::UpgradedGiftSymbol) -> Self {
        Self::UpgradedGiftSymbol(Box::new(val))
    }
}

/// TDLib `UpgradedGiftBackdropColors` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdropColors {
    /// Describes colors of a backdrop of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftBackdropColors", deserialize = "upgradedGiftBackdropColors"))]
    UpgradedGiftBackdropColors(Box<crate::types::UpgradedGiftBackdropColors>),
}

impl UpgradedGiftBackdropColors {
    /// Convenience constructor to create a [`UpgradedGiftBackdropColors::UpgradedGiftBackdropColors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_backdrop_colors(val: crate::types::UpgradedGiftBackdropColors) -> Self {
        Self::UpgradedGiftBackdropColors(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftBackdropColors`] into [`UpgradedGiftBackdropColors`].
impl From<crate::types::UpgradedGiftBackdropColors> for UpgradedGiftBackdropColors {
    fn from(val: crate::types::UpgradedGiftBackdropColors) -> Self {
        Self::UpgradedGiftBackdropColors(Box::new(val))
    }
}

/// TDLib `UpgradedGiftBackdrop` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdrop {
    /// Describes a backdrop of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftBackdrop", deserialize = "upgradedGiftBackdrop"))]
    UpgradedGiftBackdrop(Box<crate::types::UpgradedGiftBackdrop>),
}

impl UpgradedGiftBackdrop {
    /// Convenience constructor to create a [`UpgradedGiftBackdrop::UpgradedGiftBackdrop`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_backdrop(val: crate::types::UpgradedGiftBackdrop) -> Self {
        Self::UpgradedGiftBackdrop(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftBackdrop`] into [`UpgradedGiftBackdrop`].
impl From<crate::types::UpgradedGiftBackdrop> for UpgradedGiftBackdrop {
    fn from(val: crate::types::UpgradedGiftBackdrop) -> Self {
        Self::UpgradedGiftBackdrop(Box::new(val))
    }
}

/// TDLib `UpgradedGiftOriginalDetails` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftOriginalDetails {
    /// Describes the original details about the gift
    #[serde(rename(serialize = "upgradedGiftOriginalDetails", deserialize = "upgradedGiftOriginalDetails"))]
    UpgradedGiftOriginalDetails(Box<crate::types::UpgradedGiftOriginalDetails>),
}

impl UpgradedGiftOriginalDetails {
    /// Convenience constructor to create a [`UpgradedGiftOriginalDetails::UpgradedGiftOriginalDetails`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_original_details(val: crate::types::UpgradedGiftOriginalDetails) -> Self {
        Self::UpgradedGiftOriginalDetails(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftOriginalDetails`] into [`UpgradedGiftOriginalDetails`].
impl From<crate::types::UpgradedGiftOriginalDetails> for UpgradedGiftOriginalDetails {
    fn from(val: crate::types::UpgradedGiftOriginalDetails) -> Self {
        Self::UpgradedGiftOriginalDetails(Box::new(val))
    }
}

/// TDLib `UpgradedGiftColors` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftColors {
    /// Contains information about color scheme for user's name, background of empty chat photo, replies to messages and link previews
    #[serde(rename(serialize = "upgradedGiftColors", deserialize = "upgradedGiftColors"))]
    UpgradedGiftColors(Box<crate::types::UpgradedGiftColors>),
}

impl UpgradedGiftColors {
    /// Convenience constructor to create a [`UpgradedGiftColors::UpgradedGiftColors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_colors(val: crate::types::UpgradedGiftColors) -> Self {
        Self::UpgradedGiftColors(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftColors`] into [`UpgradedGiftColors`].
impl From<crate::types::UpgradedGiftColors> for UpgradedGiftColors {
    fn from(val: crate::types::UpgradedGiftColors) -> Self {
        Self::UpgradedGiftColors(Box::new(val))
    }
}

/// TDLib `Gift` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Gift {
    /// Describes a gift that can be sent to another user or channel chat
    #[serde(rename(serialize = "gift", deserialize = "gift"))]
    Gift(Box<crate::types::Gift>),
}

impl Gift {
    /// Convenience constructor to create a [`Gift::Gift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift(val: crate::types::Gift) -> Self {
        Self::Gift(Box::new(val))
    }

}

/// Converts a [`crate::types::Gift`] into [`Gift`].
impl From<crate::types::Gift> for Gift {
    fn from(val: crate::types::Gift) -> Self {
        Self::Gift(Box::new(val))
    }
}

/// TDLib `UpgradedGift` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGift {
    /// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
    #[serde(rename(serialize = "upgradedGift", deserialize = "upgradedGift"))]
    UpgradedGift(Box<crate::types::UpgradedGift>),
}

impl UpgradedGift {
    /// Convenience constructor to create a [`UpgradedGift::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::UpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGift`] into [`UpgradedGift`].
impl From<crate::types::UpgradedGift> for UpgradedGift {
    fn from(val: crate::types::UpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// TDLib `UpgradedGiftValueInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftValueInfo {
    /// Contains information about value of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftValueInfo", deserialize = "upgradedGiftValueInfo"))]
    UpgradedGiftValueInfo(Box<crate::types::UpgradedGiftValueInfo>),
}

impl UpgradedGiftValueInfo {
    /// Convenience constructor to create a [`UpgradedGiftValueInfo::UpgradedGiftValueInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_value_info(val: crate::types::UpgradedGiftValueInfo) -> Self {
        Self::UpgradedGiftValueInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftValueInfo`] into [`UpgradedGiftValueInfo`].
impl From<crate::types::UpgradedGiftValueInfo> for UpgradedGiftValueInfo {
    fn from(val: crate::types::UpgradedGiftValueInfo) -> Self {
        Self::UpgradedGiftValueInfo(Box::new(val))
    }
}

/// TDLib `UpgradeGiftResult` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradeGiftResult {
    /// Contains result of gift upgrading
    #[serde(rename(serialize = "upgradeGiftResult", deserialize = "upgradeGiftResult"))]
    UpgradeGiftResult(Box<crate::types::UpgradeGiftResult>),
}

impl UpgradeGiftResult {
    /// Convenience constructor to create a [`UpgradeGiftResult::UpgradeGiftResult`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgrade_gift_result(val: crate::types::UpgradeGiftResult) -> Self {
        Self::UpgradeGiftResult(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradeGiftResult`] into [`UpgradeGiftResult`].
impl From<crate::types::UpgradeGiftResult> for UpgradeGiftResult {
    fn from(val: crate::types::UpgradeGiftResult) -> Self {
        Self::UpgradeGiftResult(Box::new(val))
    }
}

/// Contains result of gift crafting
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CraftGiftResult {
    /// Crafting was successful
    #[serde(rename(serialize = "craftGiftResultSuccess", deserialize = "craftGiftResultSuccess"))]
    Success(Box<crate::types::CraftGiftResultSuccess>),
    /// Crafting isn't possible because one of the gifts can't be used for crafting yet
    #[serde(rename(serialize = "craftGiftResultTooEarly", deserialize = "craftGiftResultTooEarly"))]
    TooEarly(Box<crate::types::CraftGiftResultTooEarly>),
    /// Crafting isn't possible because one of the gifts isn't suitable for crafting
    #[serde(rename(serialize = "craftGiftResultInvalidGift", deserialize = "craftGiftResultInvalidGift"))]
    InvalidGift,
    /// Crafting has failed
    #[serde(rename(serialize = "craftGiftResultFail", deserialize = "craftGiftResultFail"))]
    Fail,
}

impl CraftGiftResult {
    /// Convenience constructor to create a [`CraftGiftResult::Success`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn success(val: crate::types::CraftGiftResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }

    /// Convenience constructor to create a [`CraftGiftResult::TooEarly`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn too_early(val: crate::types::CraftGiftResultTooEarly) -> Self {
        Self::TooEarly(Box::new(val))
    }

}

/// Converts a [`crate::types::CraftGiftResultSuccess`] into [`CraftGiftResult`].
impl From<crate::types::CraftGiftResultSuccess> for CraftGiftResult {
    fn from(val: crate::types::CraftGiftResultSuccess) -> Self {
        Self::Success(Box::new(val))
    }
}

/// Converts a [`crate::types::CraftGiftResultTooEarly`] into [`CraftGiftResult`].
impl From<crate::types::CraftGiftResultTooEarly> for CraftGiftResult {
    fn from(val: crate::types::CraftGiftResultTooEarly) -> Self {
        Self::TooEarly(Box::new(val))
    }
}

/// TDLib `AvailableGift` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableGift {
    /// Describes a gift that is available for purchase
    #[serde(rename(serialize = "availableGift", deserialize = "availableGift"))]
    AvailableGift(Box<crate::types::AvailableGift>),
}

impl AvailableGift {
    /// Convenience constructor to create a [`AvailableGift::AvailableGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available_gift(val: crate::types::AvailableGift) -> Self {
        Self::AvailableGift(Box::new(val))
    }

}

/// Converts a [`crate::types::AvailableGift`] into [`AvailableGift`].
impl From<crate::types::AvailableGift> for AvailableGift {
    fn from(val: crate::types::AvailableGift) -> Self {
        Self::AvailableGift(Box::new(val))
    }
}

/// TDLib `AvailableGifts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableGifts {
    /// Contains a list of gifts that can be sent to another user or channel chat
    #[serde(rename(serialize = "availableGifts", deserialize = "availableGifts"))]
    AvailableGifts(Box<crate::types::AvailableGifts>),
}

impl AvailableGifts {
    /// Convenience constructor to create a [`AvailableGifts::AvailableGifts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available_gifts(val: crate::types::AvailableGifts) -> Self {
        Self::AvailableGifts(Box::new(val))
    }

}

/// Converts a [`crate::types::AvailableGifts`] into [`AvailableGifts`].
impl From<crate::types::AvailableGifts> for AvailableGifts {
    fn from(val: crate::types::AvailableGifts) -> Self {
        Self::AvailableGifts(Box::new(val))
    }
}

/// TDLib `GiftUpgradePrice` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradePrice {
    /// Describes a price required to pay to upgrade a gift
    #[serde(rename(serialize = "giftUpgradePrice", deserialize = "giftUpgradePrice"))]
    GiftUpgradePrice(Box<crate::types::GiftUpgradePrice>),
}

impl GiftUpgradePrice {
    /// Convenience constructor to create a [`GiftUpgradePrice::GiftUpgradePrice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_upgrade_price(val: crate::types::GiftUpgradePrice) -> Self {
        Self::GiftUpgradePrice(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftUpgradePrice`] into [`GiftUpgradePrice`].
impl From<crate::types::GiftUpgradePrice> for GiftUpgradePrice {
    fn from(val: crate::types::GiftUpgradePrice) -> Self {
        Self::GiftUpgradePrice(Box::new(val))
    }
}

/// Contains identifier of an upgraded gift attribute to search for
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftAttributeId {
    /// Identifier of a gift model
    #[serde(rename(serialize = "upgradedGiftAttributeIdModel", deserialize = "upgradedGiftAttributeIdModel"))]
    Model(Box<crate::types::UpgradedGiftAttributeIdModel>),
    /// Identifier of a gift symbol
    #[serde(rename(serialize = "upgradedGiftAttributeIdSymbol", deserialize = "upgradedGiftAttributeIdSymbol"))]
    Symbol(Box<crate::types::UpgradedGiftAttributeIdSymbol>),
    /// Identifier of a gift backdrop
    #[serde(rename(serialize = "upgradedGiftAttributeIdBackdrop", deserialize = "upgradedGiftAttributeIdBackdrop"))]
    Backdrop(Box<crate::types::UpgradedGiftAttributeIdBackdrop>),
}

impl UpgradedGiftAttributeId {
    /// Convenience constructor to create a [`UpgradedGiftAttributeId::Model`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn model(val: crate::types::UpgradedGiftAttributeIdModel) -> Self {
        Self::Model(Box::new(val))
    }

    /// Convenience constructor to create a [`UpgradedGiftAttributeId::Symbol`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn symbol(val: crate::types::UpgradedGiftAttributeIdSymbol) -> Self {
        Self::Symbol(Box::new(val))
    }

    /// Convenience constructor to create a [`UpgradedGiftAttributeId::Backdrop`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn backdrop(val: crate::types::UpgradedGiftAttributeIdBackdrop) -> Self {
        Self::Backdrop(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftAttributeIdModel`] into [`UpgradedGiftAttributeId`].
impl From<crate::types::UpgradedGiftAttributeIdModel> for UpgradedGiftAttributeId {
    fn from(val: crate::types::UpgradedGiftAttributeIdModel) -> Self {
        Self::Model(Box::new(val))
    }
}

/// Converts a [`crate::types::UpgradedGiftAttributeIdSymbol`] into [`UpgradedGiftAttributeId`].
impl From<crate::types::UpgradedGiftAttributeIdSymbol> for UpgradedGiftAttributeId {
    fn from(val: crate::types::UpgradedGiftAttributeIdSymbol) -> Self {
        Self::Symbol(Box::new(val))
    }
}

/// Converts a [`crate::types::UpgradedGiftAttributeIdBackdrop`] into [`UpgradedGiftAttributeId`].
impl From<crate::types::UpgradedGiftAttributeIdBackdrop> for UpgradedGiftAttributeId {
    fn from(val: crate::types::UpgradedGiftAttributeIdBackdrop) -> Self {
        Self::Backdrop(Box::new(val))
    }
}

/// TDLib `UpgradedGiftModelCount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftModelCount {
    /// Describes a model of an upgraded gift with the number of gifts found
    #[serde(rename(serialize = "upgradedGiftModelCount", deserialize = "upgradedGiftModelCount"))]
    UpgradedGiftModelCount(Box<crate::types::UpgradedGiftModelCount>),
}

impl UpgradedGiftModelCount {
    /// Convenience constructor to create a [`UpgradedGiftModelCount::UpgradedGiftModelCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_model_count(val: crate::types::UpgradedGiftModelCount) -> Self {
        Self::UpgradedGiftModelCount(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftModelCount`] into [`UpgradedGiftModelCount`].
impl From<crate::types::UpgradedGiftModelCount> for UpgradedGiftModelCount {
    fn from(val: crate::types::UpgradedGiftModelCount) -> Self {
        Self::UpgradedGiftModelCount(Box::new(val))
    }
}

/// TDLib `UpgradedGiftSymbolCount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftSymbolCount {
    /// Describes a symbol shown on the pattern of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftSymbolCount", deserialize = "upgradedGiftSymbolCount"))]
    UpgradedGiftSymbolCount(Box<crate::types::UpgradedGiftSymbolCount>),
}

impl UpgradedGiftSymbolCount {
    /// Convenience constructor to create a [`UpgradedGiftSymbolCount::UpgradedGiftSymbolCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_symbol_count(val: crate::types::UpgradedGiftSymbolCount) -> Self {
        Self::UpgradedGiftSymbolCount(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftSymbolCount`] into [`UpgradedGiftSymbolCount`].
impl From<crate::types::UpgradedGiftSymbolCount> for UpgradedGiftSymbolCount {
    fn from(val: crate::types::UpgradedGiftSymbolCount) -> Self {
        Self::UpgradedGiftSymbolCount(Box::new(val))
    }
}

/// TDLib `UpgradedGiftBackdropCount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UpgradedGiftBackdropCount {
    /// Describes a backdrop of an upgraded gift
    #[serde(rename(serialize = "upgradedGiftBackdropCount", deserialize = "upgradedGiftBackdropCount"))]
    UpgradedGiftBackdropCount(Box<crate::types::UpgradedGiftBackdropCount>),
}

impl UpgradedGiftBackdropCount {
    /// Convenience constructor to create a [`UpgradedGiftBackdropCount::UpgradedGiftBackdropCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_backdrop_count(val: crate::types::UpgradedGiftBackdropCount) -> Self {
        Self::UpgradedGiftBackdropCount(Box::new(val))
    }

}

/// Converts a [`crate::types::UpgradedGiftBackdropCount`] into [`UpgradedGiftBackdropCount`].
impl From<crate::types::UpgradedGiftBackdropCount> for UpgradedGiftBackdropCount {
    fn from(val: crate::types::UpgradedGiftBackdropCount) -> Self {
        Self::UpgradedGiftBackdropCount(Box::new(val))
    }
}

/// Describes order in which upgraded gifts for resale will be sorted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftForResaleOrder {
    /// The gifts will be sorted by their price from the lowest to the highest
    #[serde(rename(serialize = "giftForResaleOrderPrice", deserialize = "giftForResaleOrderPrice"))]
    Price,
    /// The gifts will be sorted by the last date when their price was changed from the newest to the oldest
    #[serde(rename(serialize = "giftForResaleOrderPriceChangeDate", deserialize = "giftForResaleOrderPriceChangeDate"))]
    PriceChangeDate,
    /// The gifts will be sorted by their number from the smallest to the largest
    #[serde(rename(serialize = "giftForResaleOrderNumber", deserialize = "giftForResaleOrderNumber"))]
    Number,
}

/// TDLib `GiftForResale` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftForResale {
    /// Describes a gift available for resale
    #[serde(rename(serialize = "giftForResale", deserialize = "giftForResale"))]
    GiftForResale(Box<crate::types::GiftForResale>),
}

impl GiftForResale {
    /// Convenience constructor to create a [`GiftForResale::GiftForResale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_for_resale(val: crate::types::GiftForResale) -> Self {
        Self::GiftForResale(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftForResale`] into [`GiftForResale`].
impl From<crate::types::GiftForResale> for GiftForResale {
    fn from(val: crate::types::GiftForResale) -> Self {
        Self::GiftForResale(Box::new(val))
    }
}

/// TDLib `GiftsForResale` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftsForResale {
    /// Describes gifts available for resale
    #[serde(rename(serialize = "giftsForResale", deserialize = "giftsForResale"))]
    GiftsForResale(Box<crate::types::GiftsForResale>),
}

impl GiftsForResale {
    /// Convenience constructor to create a [`GiftsForResale::GiftsForResale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gifts_for_resale(val: crate::types::GiftsForResale) -> Self {
        Self::GiftsForResale(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftsForResale`] into [`GiftsForResale`].
impl From<crate::types::GiftsForResale> for GiftsForResale {
    fn from(val: crate::types::GiftsForResale) -> Self {
        Self::GiftsForResale(Box::new(val))
    }
}

/// Describes result of sending a resold gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftResaleResult {
    /// Operation was successfully completed
    #[serde(rename(serialize = "giftResaleResultOk", deserialize = "giftResaleResultOk"))]
    Ok(Box<crate::types::GiftResaleResultOk>),
    /// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
    #[serde(rename(serialize = "giftResaleResultPriceIncreased", deserialize = "giftResaleResultPriceIncreased"))]
    PriceIncreased(Box<crate::types::GiftResaleResultPriceIncreased>),
}

impl GiftResaleResult {
    /// Convenience constructor to create a [`GiftResaleResult::Ok`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ok(val: crate::types::GiftResaleResultOk) -> Self {
        Self::Ok(Box::new(val))
    }

    /// Convenience constructor to create a [`GiftResaleResult::PriceIncreased`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn price_increased(val: crate::types::GiftResaleResultPriceIncreased) -> Self {
        Self::PriceIncreased(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftResaleResultOk`] into [`GiftResaleResult`].
impl From<crate::types::GiftResaleResultOk> for GiftResaleResult {
    fn from(val: crate::types::GiftResaleResultOk) -> Self {
        Self::Ok(Box::new(val))
    }
}

/// Converts a [`crate::types::GiftResaleResultPriceIncreased`] into [`GiftResaleResult`].
impl From<crate::types::GiftResaleResultPriceIncreased> for GiftResaleResult {
    fn from(val: crate::types::GiftResaleResultPriceIncreased) -> Self {
        Self::PriceIncreased(Box::new(val))
    }
}

/// Represents content of a gift received by a user or a channel chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SentGift {
    /// Regular gift
    #[serde(rename(serialize = "sentGiftRegular", deserialize = "sentGiftRegular"))]
    Regular(Box<crate::types::SentGiftRegular>),
    /// Upgraded gift
    #[serde(rename(serialize = "sentGiftUpgraded", deserialize = "sentGiftUpgraded"))]
    Upgraded(Box<crate::types::SentGiftUpgraded>),
}

impl SentGift {
    /// Convenience constructor to create a [`SentGift::Regular`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn regular(val: crate::types::SentGiftRegular) -> Self {
        Self::Regular(Box::new(val))
    }

    /// Convenience constructor to create a [`SentGift::Upgraded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded(val: crate::types::SentGiftUpgraded) -> Self {
        Self::Upgraded(Box::new(val))
    }

}

/// Converts a [`crate::types::SentGiftRegular`] into [`SentGift`].
impl From<crate::types::SentGiftRegular> for SentGift {
    fn from(val: crate::types::SentGiftRegular) -> Self {
        Self::Regular(Box::new(val))
    }
}

/// Converts a [`crate::types::SentGiftUpgraded`] into [`SentGift`].
impl From<crate::types::SentGiftUpgraded> for SentGift {
    fn from(val: crate::types::SentGiftUpgraded) -> Self {
        Self::Upgraded(Box::new(val))
    }
}

/// TDLib `ReceivedGift` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReceivedGift {
    /// Represents a gift received by a user or a chat
    #[serde(rename(serialize = "receivedGift", deserialize = "receivedGift"))]
    ReceivedGift(Box<crate::types::ReceivedGift>),
}

impl ReceivedGift {
    /// Convenience constructor to create a [`ReceivedGift::ReceivedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn received_gift(val: crate::types::ReceivedGift) -> Self {
        Self::ReceivedGift(Box::new(val))
    }

}

/// Converts a [`crate::types::ReceivedGift`] into [`ReceivedGift`].
impl From<crate::types::ReceivedGift> for ReceivedGift {
    fn from(val: crate::types::ReceivedGift) -> Self {
        Self::ReceivedGift(Box::new(val))
    }
}

/// TDLib `ReceivedGifts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReceivedGifts {
    /// Represents a list of gifts received by a user or a chat
    #[serde(rename(serialize = "receivedGifts", deserialize = "receivedGifts"))]
    ReceivedGifts(Box<crate::types::ReceivedGifts>),
}

impl ReceivedGifts {
    /// Convenience constructor to create a [`ReceivedGifts::ReceivedGifts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn received_gifts(val: crate::types::ReceivedGifts) -> Self {
        Self::ReceivedGifts(Box::new(val))
    }

}

/// Converts a [`crate::types::ReceivedGifts`] into [`ReceivedGifts`].
impl From<crate::types::ReceivedGifts> for ReceivedGifts {
    fn from(val: crate::types::ReceivedGifts) -> Self {
        Self::ReceivedGifts(Box::new(val))
    }
}

/// TDLib `AttributeCraftPersistenceProbability` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AttributeCraftPersistenceProbability {
    /// Describes chance of the crafted gift to have the backdrop or symbol of one of the original gifts
    #[serde(rename(serialize = "attributeCraftPersistenceProbability", deserialize = "attributeCraftPersistenceProbability"))]
    AttributeCraftPersistenceProbability(Box<crate::types::AttributeCraftPersistenceProbability>),
}

impl AttributeCraftPersistenceProbability {
    /// Convenience constructor to create a [`AttributeCraftPersistenceProbability::AttributeCraftPersistenceProbability`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn attribute_craft_persistence_probability(val: crate::types::AttributeCraftPersistenceProbability) -> Self {
        Self::AttributeCraftPersistenceProbability(Box::new(val))
    }

}

/// Converts a [`crate::types::AttributeCraftPersistenceProbability`] into [`AttributeCraftPersistenceProbability`].
impl From<crate::types::AttributeCraftPersistenceProbability> for AttributeCraftPersistenceProbability {
    fn from(val: crate::types::AttributeCraftPersistenceProbability) -> Self {
        Self::AttributeCraftPersistenceProbability(Box::new(val))
    }
}

/// TDLib `GiftsForCrafting` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftsForCrafting {
    /// Represents a list of gifts received by a user or a chat
    #[serde(rename(serialize = "giftsForCrafting", deserialize = "giftsForCrafting"))]
    GiftsForCrafting(Box<crate::types::GiftsForCrafting>),
}

impl GiftsForCrafting {
    /// Convenience constructor to create a [`GiftsForCrafting::GiftsForCrafting`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gifts_for_crafting(val: crate::types::GiftsForCrafting) -> Self {
        Self::GiftsForCrafting(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftsForCrafting`] into [`GiftsForCrafting`].
impl From<crate::types::GiftsForCrafting> for GiftsForCrafting {
    fn from(val: crate::types::GiftsForCrafting) -> Self {
        Self::GiftsForCrafting(Box::new(val))
    }
}

/// TDLib `GiftUpgradePreview` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradePreview {
    /// Contains examples of possible upgraded gifts for the given regular gift
    #[serde(rename(serialize = "giftUpgradePreview", deserialize = "giftUpgradePreview"))]
    GiftUpgradePreview(Box<crate::types::GiftUpgradePreview>),
}

impl GiftUpgradePreview {
    /// Convenience constructor to create a [`GiftUpgradePreview::GiftUpgradePreview`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_upgrade_preview(val: crate::types::GiftUpgradePreview) -> Self {
        Self::GiftUpgradePreview(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftUpgradePreview`] into [`GiftUpgradePreview`].
impl From<crate::types::GiftUpgradePreview> for GiftUpgradePreview {
    fn from(val: crate::types::GiftUpgradePreview) -> Self {
        Self::GiftUpgradePreview(Box::new(val))
    }
}

/// TDLib `GiftUpgradeVariants` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftUpgradeVariants {
    /// Contains all possible variants of upgraded gifts for the given regular gift
    #[serde(rename(serialize = "giftUpgradeVariants", deserialize = "giftUpgradeVariants"))]
    GiftUpgradeVariants(Box<crate::types::GiftUpgradeVariants>),
}

impl GiftUpgradeVariants {
    /// Convenience constructor to create a [`GiftUpgradeVariants::GiftUpgradeVariants`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_upgrade_variants(val: crate::types::GiftUpgradeVariants) -> Self {
        Self::GiftUpgradeVariants(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftUpgradeVariants`] into [`GiftUpgradeVariants`].
impl From<crate::types::GiftUpgradeVariants> for GiftUpgradeVariants {
    fn from(val: crate::types::GiftUpgradeVariants) -> Self {
        Self::GiftUpgradeVariants(Box::new(val))
    }
}

/// TDLib `AuctionBid` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionBid {
    /// Describes a bid in an auction
    #[serde(rename(serialize = "auctionBid", deserialize = "auctionBid"))]
    AuctionBid(Box<crate::types::AuctionBid>),
}

impl AuctionBid {
    /// Convenience constructor to create a [`AuctionBid::AuctionBid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn auction_bid(val: crate::types::AuctionBid) -> Self {
        Self::AuctionBid(Box::new(val))
    }

}

/// Converts a [`crate::types::AuctionBid`] into [`AuctionBid`].
impl From<crate::types::AuctionBid> for AuctionBid {
    fn from(val: crate::types::AuctionBid) -> Self {
        Self::AuctionBid(Box::new(val))
    }
}

/// TDLib `AuctionRound` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionRound {
    /// Describes a round of an auction
    #[serde(rename(serialize = "auctionRound", deserialize = "auctionRound"))]
    AuctionRound(Box<crate::types::AuctionRound>),
}

impl AuctionRound {
    /// Convenience constructor to create a [`AuctionRound::AuctionRound`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn auction_round(val: crate::types::AuctionRound) -> Self {
        Self::AuctionRound(Box::new(val))
    }

}

/// Converts a [`crate::types::AuctionRound`] into [`AuctionRound`].
impl From<crate::types::AuctionRound> for AuctionRound {
    fn from(val: crate::types::AuctionRound) -> Self {
        Self::AuctionRound(Box::new(val))
    }
}

/// Describes state of an auction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuctionState {
    /// Contains information about an ongoing or scheduled auction
    #[serde(rename(serialize = "auctionStateActive", deserialize = "auctionStateActive"))]
    Active(Box<crate::types::AuctionStateActive>),
    /// Contains information about a finished auction
    #[serde(rename(serialize = "auctionStateFinished", deserialize = "auctionStateFinished"))]
    Finished(Box<crate::types::AuctionStateFinished>),
}

impl AuctionState {
    /// Convenience constructor to create a [`AuctionState::Active`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn active(val: crate::types::AuctionStateActive) -> Self {
        Self::Active(Box::new(val))
    }

    /// Convenience constructor to create a [`AuctionState::Finished`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn finished(val: crate::types::AuctionStateFinished) -> Self {
        Self::Finished(Box::new(val))
    }

}

/// Converts a [`crate::types::AuctionStateActive`] into [`AuctionState`].
impl From<crate::types::AuctionStateActive> for AuctionState {
    fn from(val: crate::types::AuctionStateActive) -> Self {
        Self::Active(Box::new(val))
    }
}

/// Converts a [`crate::types::AuctionStateFinished`] into [`AuctionState`].
impl From<crate::types::AuctionStateFinished> for AuctionState {
    fn from(val: crate::types::AuctionStateFinished) -> Self {
        Self::Finished(Box::new(val))
    }
}

/// TDLib `GiftAuctionState` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionState {
    /// Represent auction state of a gift
    #[serde(rename(serialize = "giftAuctionState", deserialize = "giftAuctionState"))]
    GiftAuctionState(Box<crate::types::GiftAuctionState>),
}

impl GiftAuctionState {
    /// Convenience constructor to create a [`GiftAuctionState::GiftAuctionState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction_state(val: crate::types::GiftAuctionState) -> Self {
        Self::GiftAuctionState(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftAuctionState`] into [`GiftAuctionState`].
impl From<crate::types::GiftAuctionState> for GiftAuctionState {
    fn from(val: crate::types::GiftAuctionState) -> Self {
        Self::GiftAuctionState(Box::new(val))
    }
}

/// TDLib `GiftAuctionAcquiredGift` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionAcquiredGift {
    /// Represents a gift that was acquired by the current user on an auction
    #[serde(rename(serialize = "giftAuctionAcquiredGift", deserialize = "giftAuctionAcquiredGift"))]
    GiftAuctionAcquiredGift(Box<crate::types::GiftAuctionAcquiredGift>),
}

impl GiftAuctionAcquiredGift {
    /// Convenience constructor to create a [`GiftAuctionAcquiredGift::GiftAuctionAcquiredGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction_acquired_gift(val: crate::types::GiftAuctionAcquiredGift) -> Self {
        Self::GiftAuctionAcquiredGift(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftAuctionAcquiredGift`] into [`GiftAuctionAcquiredGift`].
impl From<crate::types::GiftAuctionAcquiredGift> for GiftAuctionAcquiredGift {
    fn from(val: crate::types::GiftAuctionAcquiredGift) -> Self {
        Self::GiftAuctionAcquiredGift(Box::new(val))
    }
}

/// TDLib `GiftAuctionAcquiredGifts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftAuctionAcquiredGifts {
    /// Represents a list of gifts that were acquired by the current user on an auction
    #[serde(rename(serialize = "giftAuctionAcquiredGifts", deserialize = "giftAuctionAcquiredGifts"))]
    GiftAuctionAcquiredGifts(Box<crate::types::GiftAuctionAcquiredGifts>),
}

impl GiftAuctionAcquiredGifts {
    /// Convenience constructor to create a [`GiftAuctionAcquiredGifts::GiftAuctionAcquiredGifts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction_acquired_gifts(val: crate::types::GiftAuctionAcquiredGifts) -> Self {
        Self::GiftAuctionAcquiredGifts(Box::new(val))
    }

}

/// Converts a [`crate::types::GiftAuctionAcquiredGifts`] into [`GiftAuctionAcquiredGifts`].
impl From<crate::types::GiftAuctionAcquiredGifts> for GiftAuctionAcquiredGifts {
    fn from(val: crate::types::GiftAuctionAcquiredGifts) -> Self {
        Self::GiftAuctionAcquiredGifts(Box::new(val))
    }
}

/// Describes direction of transactions in a transaction list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TransactionDirection {
    /// The transaction is incoming and increases the amount of owned currency
    #[serde(rename(serialize = "transactionDirectionIncoming", deserialize = "transactionDirectionIncoming"))]
    Incoming,
    /// The transaction is outgoing and decreases the amount of owned currency
    #[serde(rename(serialize = "transactionDirectionOutgoing", deserialize = "transactionDirectionOutgoing"))]
    Outgoing,
}

/// Describes type of transaction with Telegram Stars
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransactionType {
    /// The transaction is a deposit of Telegram Stars from the Premium bot; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePremiumBotDeposit", deserialize = "starTransactionTypePremiumBotDeposit"))]
    PremiumBotDeposit,
    /// The transaction is a deposit of Telegram Stars from App Store; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeAppStoreDeposit", deserialize = "starTransactionTypeAppStoreDeposit"))]
    AppStoreDeposit,
    /// The transaction is a deposit of Telegram Stars from Google Play; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGooglePlayDeposit", deserialize = "starTransactionTypeGooglePlayDeposit"))]
    GooglePlayDeposit,
    /// The transaction is a deposit of Telegram Stars from Fragment; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypeFragmentDeposit", deserialize = "starTransactionTypeFragmentDeposit"))]
    FragmentDeposit,
    /// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUserDeposit", deserialize = "starTransactionTypeUserDeposit"))]
    UserDeposit(Box<crate::types::StarTransactionTypeUserDeposit>),
    /// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiveawayDeposit", deserialize = "starTransactionTypeGiveawayDeposit"))]
    GiveawayDeposit(Box<crate::types::StarTransactionTypeGiveawayDeposit>),
    /// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
    #[serde(rename(serialize = "starTransactionTypeFragmentWithdrawal", deserialize = "starTransactionTypeFragmentWithdrawal"))]
    FragmentWithdrawal(Box<crate::types::StarTransactionTypeFragmentWithdrawal>),
    /// The transaction is a withdrawal of earned Telegram Stars to Telegram Ad platform; relevant for bots and channel chats only
    #[serde(rename(serialize = "starTransactionTypeTelegramAdsWithdrawal", deserialize = "starTransactionTypeTelegramAdsWithdrawal"))]
    TelegramAdsWithdrawal,
    /// The transaction is a payment for Telegram API usage; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeTelegramApiUsage", deserialize = "starTransactionTypeTelegramApiUsage"))]
    TelegramApiUsage(Box<crate::types::StarTransactionTypeTelegramApiUsage>),
    /// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotPaidMediaPurchase", deserialize = "starTransactionTypeBotPaidMediaPurchase"))]
    BotPaidMediaPurchase(Box<crate::types::StarTransactionTypeBotPaidMediaPurchase>),
    /// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotPaidMediaSale", deserialize = "starTransactionTypeBotPaidMediaSale"))]
    BotPaidMediaSale(Box<crate::types::StarTransactionTypeBotPaidMediaSale>),
    /// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidMediaPurchase", deserialize = "starTransactionTypeChannelPaidMediaPurchase"))]
    ChannelPaidMediaPurchase(Box<crate::types::StarTransactionTypeChannelPaidMediaPurchase>),
    /// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidMediaSale", deserialize = "starTransactionTypeChannelPaidMediaSale"))]
    ChannelPaidMediaSale(Box<crate::types::StarTransactionTypeChannelPaidMediaSale>),
    /// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotInvoicePurchase", deserialize = "starTransactionTypeBotInvoicePurchase"))]
    BotInvoicePurchase(Box<crate::types::StarTransactionTypeBotInvoicePurchase>),
    /// The transaction is a sale of a product by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotInvoiceSale", deserialize = "starTransactionTypeBotInvoiceSale"))]
    BotInvoiceSale(Box<crate::types::StarTransactionTypeBotInvoiceSale>),
    /// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBotSubscriptionPurchase", deserialize = "starTransactionTypeBotSubscriptionPurchase"))]
    BotSubscriptionPurchase(Box<crate::types::StarTransactionTypeBotSubscriptionPurchase>),
    /// The transaction is a sale of a subscription by the bot; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBotSubscriptionSale", deserialize = "starTransactionTypeBotSubscriptionSale"))]
    BotSubscriptionSale(Box<crate::types::StarTransactionTypeBotSubscriptionSale>),
    /// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelSubscriptionPurchase", deserialize = "starTransactionTypeChannelSubscriptionPurchase"))]
    ChannelSubscriptionPurchase(Box<crate::types::StarTransactionTypeChannelSubscriptionPurchase>),
    /// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelSubscriptionSale", deserialize = "starTransactionTypeChannelSubscriptionSale"))]
    ChannelSubscriptionSale(Box<crate::types::StarTransactionTypeChannelSubscriptionSale>),
    /// The transaction is a bid on a gift auction; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftAuctionBid", deserialize = "starTransactionTypeGiftAuctionBid"))]
    GiftAuctionBid(Box<crate::types::StarTransactionTypeGiftAuctionBid>),
    /// The transaction is a purchase of a regular gift; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypeGiftPurchase", deserialize = "starTransactionTypeGiftPurchase"))]
    GiftPurchase(Box<crate::types::StarTransactionTypeGiftPurchase>),
    /// The transaction is an offer of gift purchase; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftPurchaseOffer", deserialize = "starTransactionTypeGiftPurchaseOffer"))]
    GiftPurchaseOffer(Box<crate::types::StarTransactionTypeGiftPurchaseOffer>),
    /// The transaction is a transfer of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftTransfer", deserialize = "starTransactionTypeGiftTransfer"))]
    GiftTransfer(Box<crate::types::StarTransactionTypeGiftTransfer>),
    /// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftOriginalDetailsDrop", deserialize = "starTransactionTypeGiftOriginalDetailsDrop"))]
    GiftOriginalDetailsDrop(Box<crate::types::StarTransactionTypeGiftOriginalDetailsDrop>),
    /// The transaction is a sale of a received gift; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypeGiftSale", deserialize = "starTransactionTypeGiftSale"))]
    GiftSale(Box<crate::types::StarTransactionTypeGiftSale>),
    /// The transaction is an upgrade of a gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftUpgrade", deserialize = "starTransactionTypeGiftUpgrade"))]
    GiftUpgrade(Box<crate::types::StarTransactionTypeGiftUpgrade>),
    /// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeGiftUpgradePurchase", deserialize = "starTransactionTypeGiftUpgradePurchase"))]
    GiftUpgradePurchase(Box<crate::types::StarTransactionTypeGiftUpgradePurchase>),
    /// The transaction is a purchase of an upgraded gift for some user or channel; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUpgradedGiftPurchase", deserialize = "starTransactionTypeUpgradedGiftPurchase"))]
    UpgradedGiftPurchase(Box<crate::types::StarTransactionTypeUpgradedGiftPurchase>),
    /// The transaction is a sale of an upgraded gift; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeUpgradedGiftSale", deserialize = "starTransactionTypeUpgradedGiftSale"))]
    UpgradedGiftSale(Box<crate::types::StarTransactionTypeUpgradedGiftSale>),
    /// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidReactionSend", deserialize = "starTransactionTypeChannelPaidReactionSend"))]
    ChannelPaidReactionSend(Box<crate::types::StarTransactionTypeChannelPaidReactionSend>),
    /// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeChannelPaidReactionReceive", deserialize = "starTransactionTypeChannelPaidReactionReceive"))]
    ChannelPaidReactionReceive(Box<crate::types::StarTransactionTypeChannelPaidReactionReceive>),
    /// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
    #[serde(rename(serialize = "starTransactionTypeAffiliateProgramCommission", deserialize = "starTransactionTypeAffiliateProgramCommission"))]
    AffiliateProgramCommission(Box<crate::types::StarTransactionTypeAffiliateProgramCommission>),
    /// The transaction is a sending of a paid message; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidMessageSend", deserialize = "starTransactionTypePaidMessageSend"))]
    PaidMessageSend(Box<crate::types::StarTransactionTypePaidMessageSend>),
    /// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidMessageReceive", deserialize = "starTransactionTypePaidMessageReceive"))]
    PaidMessageReceive(Box<crate::types::StarTransactionTypePaidMessageReceive>),
    /// The transaction is a sending of a paid group call message; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallMessageSend", deserialize = "starTransactionTypePaidGroupCallMessageSend"))]
    PaidGroupCallMessageSend(Box<crate::types::StarTransactionTypePaidGroupCallMessageSend>),
    /// The transaction is a receiving of a paid group call message; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallMessageReceive", deserialize = "starTransactionTypePaidGroupCallMessageReceive"))]
    PaidGroupCallMessageReceive(Box<crate::types::StarTransactionTypePaidGroupCallMessageReceive>),
    /// The transaction is a sending of a paid group reaction; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallReactionSend", deserialize = "starTransactionTypePaidGroupCallReactionSend"))]
    PaidGroupCallReactionSend(Box<crate::types::StarTransactionTypePaidGroupCallReactionSend>),
    /// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
    #[serde(rename(serialize = "starTransactionTypePaidGroupCallReactionReceive", deserialize = "starTransactionTypePaidGroupCallReactionReceive"))]
    PaidGroupCallReactionReceive(Box<crate::types::StarTransactionTypePaidGroupCallReactionReceive>),
    /// The transaction is a payment for a suggested post; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeSuggestedPostPaymentSend", deserialize = "starTransactionTypeSuggestedPostPaymentSend"))]
    SuggestedPostPaymentSend(Box<crate::types::StarTransactionTypeSuggestedPostPaymentSend>),
    /// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
    #[serde(rename(serialize = "starTransactionTypeSuggestedPostPaymentReceive", deserialize = "starTransactionTypeSuggestedPostPaymentReceive"))]
    SuggestedPostPaymentReceive(Box<crate::types::StarTransactionTypeSuggestedPostPaymentReceive>),
    /// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
    #[serde(rename(serialize = "starTransactionTypePremiumPurchase", deserialize = "starTransactionTypePremiumPurchase"))]
    PremiumPurchase(Box<crate::types::StarTransactionTypePremiumPurchase>),
    /// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypeBusinessBotTransferSend", deserialize = "starTransactionTypeBusinessBotTransferSend"))]
    BusinessBotTransferSend(Box<crate::types::StarTransactionTypeBusinessBotTransferSend>),
    /// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
    #[serde(rename(serialize = "starTransactionTypeBusinessBotTransferReceive", deserialize = "starTransactionTypeBusinessBotTransferReceive"))]
    BusinessBotTransferReceive(Box<crate::types::StarTransactionTypeBusinessBotTransferReceive>),
    /// The transaction is a payment for search of posts in public Telegram channels; relevant for regular users only
    #[serde(rename(serialize = "starTransactionTypePublicPostSearch", deserialize = "starTransactionTypePublicPostSearch"))]
    PublicPostSearch,
    /// The transaction is a transaction of an unsupported type
    #[serde(rename(serialize = "starTransactionTypeUnsupported", deserialize = "starTransactionTypeUnsupported"))]
    Unsupported,
}

impl StarTransactionType {
    /// Convenience constructor to create a [`StarTransactionType::UserDeposit`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_deposit(val: crate::types::StarTransactionTypeUserDeposit) -> Self {
        Self::UserDeposit(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiveawayDeposit`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn giveaway_deposit(val: crate::types::StarTransactionTypeGiveawayDeposit) -> Self {
        Self::GiveawayDeposit(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::FragmentWithdrawal`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment_withdrawal(val: crate::types::StarTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::TelegramApiUsage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn telegram_api_usage(val: crate::types::StarTransactionTypeTelegramApiUsage) -> Self {
        Self::TelegramApiUsage(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotPaidMediaPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_paid_media_purchase(val: crate::types::StarTransactionTypeBotPaidMediaPurchase) -> Self {
        Self::BotPaidMediaPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotPaidMediaSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_paid_media_sale(val: crate::types::StarTransactionTypeBotPaidMediaSale) -> Self {
        Self::BotPaidMediaSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelPaidMediaPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_paid_media_purchase(val: crate::types::StarTransactionTypeChannelPaidMediaPurchase) -> Self {
        Self::ChannelPaidMediaPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelPaidMediaSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_paid_media_sale(val: crate::types::StarTransactionTypeChannelPaidMediaSale) -> Self {
        Self::ChannelPaidMediaSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotInvoicePurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_invoice_purchase(val: crate::types::StarTransactionTypeBotInvoicePurchase) -> Self {
        Self::BotInvoicePurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotInvoiceSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_invoice_sale(val: crate::types::StarTransactionTypeBotInvoiceSale) -> Self {
        Self::BotInvoiceSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotSubscriptionPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_subscription_purchase(val: crate::types::StarTransactionTypeBotSubscriptionPurchase) -> Self {
        Self::BotSubscriptionPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BotSubscriptionSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_subscription_sale(val: crate::types::StarTransactionTypeBotSubscriptionSale) -> Self {
        Self::BotSubscriptionSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelSubscriptionPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_subscription_purchase(val: crate::types::StarTransactionTypeChannelSubscriptionPurchase) -> Self {
        Self::ChannelSubscriptionPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelSubscriptionSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_subscription_sale(val: crate::types::StarTransactionTypeChannelSubscriptionSale) -> Self {
        Self::ChannelSubscriptionSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftAuctionBid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction_bid(val: crate::types::StarTransactionTypeGiftAuctionBid) -> Self {
        Self::GiftAuctionBid(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_purchase(val: crate::types::StarTransactionTypeGiftPurchase) -> Self {
        Self::GiftPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftPurchaseOffer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_purchase_offer(val: crate::types::StarTransactionTypeGiftPurchaseOffer) -> Self {
        Self::GiftPurchaseOffer(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftTransfer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_transfer(val: crate::types::StarTransactionTypeGiftTransfer) -> Self {
        Self::GiftTransfer(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftOriginalDetailsDrop`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_original_details_drop(val: crate::types::StarTransactionTypeGiftOriginalDetailsDrop) -> Self {
        Self::GiftOriginalDetailsDrop(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_sale(val: crate::types::StarTransactionTypeGiftSale) -> Self {
        Self::GiftSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftUpgrade`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_upgrade(val: crate::types::StarTransactionTypeGiftUpgrade) -> Self {
        Self::GiftUpgrade(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::GiftUpgradePurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_upgrade_purchase(val: crate::types::StarTransactionTypeGiftUpgradePurchase) -> Self {
        Self::GiftUpgradePurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::UpgradedGiftPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_purchase(val: crate::types::StarTransactionTypeUpgradedGiftPurchase) -> Self {
        Self::UpgradedGiftPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::UpgradedGiftSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_sale(val: crate::types::StarTransactionTypeUpgradedGiftSale) -> Self {
        Self::UpgradedGiftSale(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelPaidReactionSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_paid_reaction_send(val: crate::types::StarTransactionTypeChannelPaidReactionSend) -> Self {
        Self::ChannelPaidReactionSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::ChannelPaidReactionReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_paid_reaction_receive(val: crate::types::StarTransactionTypeChannelPaidReactionReceive) -> Self {
        Self::ChannelPaidReactionReceive(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::AffiliateProgramCommission`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn affiliate_program_commission(val: crate::types::StarTransactionTypeAffiliateProgramCommission) -> Self {
        Self::AffiliateProgramCommission(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidMessageSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_message_send(val: crate::types::StarTransactionTypePaidMessageSend) -> Self {
        Self::PaidMessageSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidMessageReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_message_receive(val: crate::types::StarTransactionTypePaidMessageReceive) -> Self {
        Self::PaidMessageReceive(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidGroupCallMessageSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_group_call_message_send(val: crate::types::StarTransactionTypePaidGroupCallMessageSend) -> Self {
        Self::PaidGroupCallMessageSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidGroupCallMessageReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_group_call_message_receive(val: crate::types::StarTransactionTypePaidGroupCallMessageReceive) -> Self {
        Self::PaidGroupCallMessageReceive(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidGroupCallReactionSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_group_call_reaction_send(val: crate::types::StarTransactionTypePaidGroupCallReactionSend) -> Self {
        Self::PaidGroupCallReactionSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PaidGroupCallReactionReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_group_call_reaction_receive(val: crate::types::StarTransactionTypePaidGroupCallReactionReceive) -> Self {
        Self::PaidGroupCallReactionReceive(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::SuggestedPostPaymentSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_post_payment_send(val: crate::types::StarTransactionTypeSuggestedPostPaymentSend) -> Self {
        Self::SuggestedPostPaymentSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::SuggestedPostPaymentReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_post_payment_receive(val: crate::types::StarTransactionTypeSuggestedPostPaymentReceive) -> Self {
        Self::SuggestedPostPaymentReceive(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::PremiumPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_purchase(val: crate::types::StarTransactionTypePremiumPurchase) -> Self {
        Self::PremiumPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BusinessBotTransferSend`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_bot_transfer_send(val: crate::types::StarTransactionTypeBusinessBotTransferSend) -> Self {
        Self::BusinessBotTransferSend(Box::new(val))
    }

    /// Convenience constructor to create a [`StarTransactionType::BusinessBotTransferReceive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_bot_transfer_receive(val: crate::types::StarTransactionTypeBusinessBotTransferReceive) -> Self {
        Self::BusinessBotTransferReceive(Box::new(val))
    }

}

/// Converts a [`crate::types::StarTransactionTypeUserDeposit`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeUserDeposit> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeUserDeposit) -> Self {
        Self::UserDeposit(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiveawayDeposit`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiveawayDeposit> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiveawayDeposit) -> Self {
        Self::GiveawayDeposit(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeFragmentWithdrawal`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeFragmentWithdrawal> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeTelegramApiUsage`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeTelegramApiUsage> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeTelegramApiUsage) -> Self {
        Self::TelegramApiUsage(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotPaidMediaPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotPaidMediaPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotPaidMediaPurchase) -> Self {
        Self::BotPaidMediaPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotPaidMediaSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotPaidMediaSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotPaidMediaSale) -> Self {
        Self::BotPaidMediaSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelPaidMediaPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelPaidMediaPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelPaidMediaPurchase) -> Self {
        Self::ChannelPaidMediaPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelPaidMediaSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelPaidMediaSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelPaidMediaSale) -> Self {
        Self::ChannelPaidMediaSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotInvoicePurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotInvoicePurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotInvoicePurchase) -> Self {
        Self::BotInvoicePurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotInvoiceSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotInvoiceSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotInvoiceSale) -> Self {
        Self::BotInvoiceSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotSubscriptionPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotSubscriptionPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotSubscriptionPurchase) -> Self {
        Self::BotSubscriptionPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBotSubscriptionSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBotSubscriptionSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBotSubscriptionSale) -> Self {
        Self::BotSubscriptionSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelSubscriptionPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelSubscriptionPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelSubscriptionPurchase) -> Self {
        Self::ChannelSubscriptionPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelSubscriptionSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelSubscriptionSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelSubscriptionSale) -> Self {
        Self::ChannelSubscriptionSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftAuctionBid`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftAuctionBid> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftAuctionBid) -> Self {
        Self::GiftAuctionBid(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftPurchase) -> Self {
        Self::GiftPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftPurchaseOffer`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftPurchaseOffer> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftPurchaseOffer) -> Self {
        Self::GiftPurchaseOffer(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftTransfer`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftTransfer> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftTransfer) -> Self {
        Self::GiftTransfer(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftOriginalDetailsDrop`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftOriginalDetailsDrop> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftOriginalDetailsDrop) -> Self {
        Self::GiftOriginalDetailsDrop(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftSale) -> Self {
        Self::GiftSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftUpgrade`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftUpgrade> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftUpgrade) -> Self {
        Self::GiftUpgrade(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeGiftUpgradePurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeGiftUpgradePurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeGiftUpgradePurchase) -> Self {
        Self::GiftUpgradePurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeUpgradedGiftPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeUpgradedGiftPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeUpgradedGiftPurchase) -> Self {
        Self::UpgradedGiftPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeUpgradedGiftSale`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeUpgradedGiftSale> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeUpgradedGiftSale) -> Self {
        Self::UpgradedGiftSale(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelPaidReactionSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelPaidReactionSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelPaidReactionSend) -> Self {
        Self::ChannelPaidReactionSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeChannelPaidReactionReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeChannelPaidReactionReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeChannelPaidReactionReceive) -> Self {
        Self::ChannelPaidReactionReceive(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeAffiliateProgramCommission`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeAffiliateProgramCommission> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeAffiliateProgramCommission) -> Self {
        Self::AffiliateProgramCommission(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidMessageSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidMessageSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidMessageSend) -> Self {
        Self::PaidMessageSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidMessageReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidMessageReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidMessageReceive) -> Self {
        Self::PaidMessageReceive(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidGroupCallMessageSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidGroupCallMessageSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidGroupCallMessageSend) -> Self {
        Self::PaidGroupCallMessageSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidGroupCallMessageReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidGroupCallMessageReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidGroupCallMessageReceive) -> Self {
        Self::PaidGroupCallMessageReceive(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidGroupCallReactionSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidGroupCallReactionSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidGroupCallReactionSend) -> Self {
        Self::PaidGroupCallReactionSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePaidGroupCallReactionReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePaidGroupCallReactionReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePaidGroupCallReactionReceive) -> Self {
        Self::PaidGroupCallReactionReceive(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeSuggestedPostPaymentSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeSuggestedPostPaymentSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeSuggestedPostPaymentSend) -> Self {
        Self::SuggestedPostPaymentSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeSuggestedPostPaymentReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeSuggestedPostPaymentReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeSuggestedPostPaymentReceive) -> Self {
        Self::SuggestedPostPaymentReceive(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypePremiumPurchase`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypePremiumPurchase> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypePremiumPurchase) -> Self {
        Self::PremiumPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBusinessBotTransferSend`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBusinessBotTransferSend> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBusinessBotTransferSend) -> Self {
        Self::BusinessBotTransferSend(Box::new(val))
    }
}

/// Converts a [`crate::types::StarTransactionTypeBusinessBotTransferReceive`] into [`StarTransactionType`].
impl From<crate::types::StarTransactionTypeBusinessBotTransferReceive> for StarTransactionType {
    fn from(val: crate::types::StarTransactionTypeBusinessBotTransferReceive) -> Self {
        Self::BusinessBotTransferReceive(Box::new(val))
    }
}

/// TDLib `StarTransaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransaction {
    /// Represents a transaction changing the amount of owned Telegram Stars
    #[serde(rename(serialize = "starTransaction", deserialize = "starTransaction"))]
    StarTransaction(Box<crate::types::StarTransaction>),
}

impl StarTransaction {
    /// Convenience constructor to create a [`StarTransaction::StarTransaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_transaction(val: crate::types::StarTransaction) -> Self {
        Self::StarTransaction(Box::new(val))
    }

}

/// Converts a [`crate::types::StarTransaction`] into [`StarTransaction`].
impl From<crate::types::StarTransaction> for StarTransaction {
    fn from(val: crate::types::StarTransaction) -> Self {
        Self::StarTransaction(Box::new(val))
    }
}

/// TDLib `StarTransactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarTransactions {
    /// Represents a list of Telegram Star transactions
    #[serde(rename(serialize = "starTransactions", deserialize = "starTransactions"))]
    StarTransactions(Box<crate::types::StarTransactions>),
}

impl StarTransactions {
    /// Convenience constructor to create a [`StarTransactions::StarTransactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_transactions(val: crate::types::StarTransactions) -> Self {
        Self::StarTransactions(Box::new(val))
    }

}

/// Converts a [`crate::types::StarTransactions`] into [`StarTransactions`].
impl From<crate::types::StarTransactions> for StarTransactions {
    fn from(val: crate::types::StarTransactions) -> Self {
        Self::StarTransactions(Box::new(val))
    }
}

/// Describes type of transaction with TON Grams
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransactionType {
    /// The transaction is a deposit of Grams from Fragment
    #[serde(rename(serialize = "tonTransactionTypeFragmentDeposit", deserialize = "tonTransactionTypeFragmentDeposit"))]
    FragmentDeposit(Box<crate::types::TonTransactionTypeFragmentDeposit>),
    /// The transaction is a withdrawal of earned Grams to Fragment
    #[serde(rename(serialize = "tonTransactionTypeFragmentWithdrawal", deserialize = "tonTransactionTypeFragmentWithdrawal"))]
    FragmentWithdrawal(Box<crate::types::TonTransactionTypeFragmentWithdrawal>),
    /// The transaction is a payment for a suggested post
    #[serde(rename(serialize = "tonTransactionTypeSuggestedPostPayment", deserialize = "tonTransactionTypeSuggestedPostPayment"))]
    SuggestedPostPayment(Box<crate::types::TonTransactionTypeSuggestedPostPayment>),
    /// The transaction is an offer of gift purchase
    #[serde(rename(serialize = "tonTransactionTypeGiftPurchaseOffer", deserialize = "tonTransactionTypeGiftPurchaseOffer"))]
    GiftPurchaseOffer(Box<crate::types::TonTransactionTypeGiftPurchaseOffer>),
    /// The transaction is a purchase of an upgraded gift for some user or channel
    #[serde(rename(serialize = "tonTransactionTypeUpgradedGiftPurchase", deserialize = "tonTransactionTypeUpgradedGiftPurchase"))]
    UpgradedGiftPurchase(Box<crate::types::TonTransactionTypeUpgradedGiftPurchase>),
    /// The transaction is a sale of an upgraded gift
    #[serde(rename(serialize = "tonTransactionTypeUpgradedGiftSale", deserialize = "tonTransactionTypeUpgradedGiftSale"))]
    UpgradedGiftSale(Box<crate::types::TonTransactionTypeUpgradedGiftSale>),
    /// The transaction is a payment for stake dice throw
    #[serde(rename(serialize = "tonTransactionTypeStakeDiceStake", deserialize = "tonTransactionTypeStakeDiceStake"))]
    StakeDiceStake,
    /// The transaction is a payment for successful stake dice throw
    #[serde(rename(serialize = "tonTransactionTypeStakeDicePayout", deserialize = "tonTransactionTypeStakeDicePayout"))]
    StakeDicePayout,
    /// The transaction is a transaction of an unsupported type
    #[serde(rename(serialize = "tonTransactionTypeUnsupported", deserialize = "tonTransactionTypeUnsupported"))]
    Unsupported,
}

impl TonTransactionType {
    /// Convenience constructor to create a [`TonTransactionType::FragmentDeposit`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment_deposit(val: crate::types::TonTransactionTypeFragmentDeposit) -> Self {
        Self::FragmentDeposit(Box::new(val))
    }

    /// Convenience constructor to create a [`TonTransactionType::FragmentWithdrawal`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fragment_withdrawal(val: crate::types::TonTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }

    /// Convenience constructor to create a [`TonTransactionType::SuggestedPostPayment`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_post_payment(val: crate::types::TonTransactionTypeSuggestedPostPayment) -> Self {
        Self::SuggestedPostPayment(Box::new(val))
    }

    /// Convenience constructor to create a [`TonTransactionType::GiftPurchaseOffer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_purchase_offer(val: crate::types::TonTransactionTypeGiftPurchaseOffer) -> Self {
        Self::GiftPurchaseOffer(Box::new(val))
    }

    /// Convenience constructor to create a [`TonTransactionType::UpgradedGiftPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_purchase(val: crate::types::TonTransactionTypeUpgradedGiftPurchase) -> Self {
        Self::UpgradedGiftPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`TonTransactionType::UpgradedGiftSale`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift_sale(val: crate::types::TonTransactionTypeUpgradedGiftSale) -> Self {
        Self::UpgradedGiftSale(Box::new(val))
    }

}

/// Converts a [`crate::types::TonTransactionTypeFragmentDeposit`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeFragmentDeposit> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeFragmentDeposit) -> Self {
        Self::FragmentDeposit(Box::new(val))
    }
}

/// Converts a [`crate::types::TonTransactionTypeFragmentWithdrawal`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeFragmentWithdrawal> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeFragmentWithdrawal) -> Self {
        Self::FragmentWithdrawal(Box::new(val))
    }
}

/// Converts a [`crate::types::TonTransactionTypeSuggestedPostPayment`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeSuggestedPostPayment> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeSuggestedPostPayment) -> Self {
        Self::SuggestedPostPayment(Box::new(val))
    }
}

/// Converts a [`crate::types::TonTransactionTypeGiftPurchaseOffer`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeGiftPurchaseOffer> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeGiftPurchaseOffer) -> Self {
        Self::GiftPurchaseOffer(Box::new(val))
    }
}

/// Converts a [`crate::types::TonTransactionTypeUpgradedGiftPurchase`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeUpgradedGiftPurchase> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeUpgradedGiftPurchase) -> Self {
        Self::UpgradedGiftPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::TonTransactionTypeUpgradedGiftSale`] into [`TonTransactionType`].
impl From<crate::types::TonTransactionTypeUpgradedGiftSale> for TonTransactionType {
    fn from(val: crate::types::TonTransactionTypeUpgradedGiftSale) -> Self {
        Self::UpgradedGiftSale(Box::new(val))
    }
}

/// TDLib `TonTransaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransaction {
    /// Represents a transaction changing the amount of owned TON Grams
    #[serde(rename(serialize = "tonTransaction", deserialize = "tonTransaction"))]
    TonTransaction(Box<crate::types::TonTransaction>),
}

impl TonTransaction {
    /// Convenience constructor to create a [`TonTransaction::TonTransaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ton_transaction(val: crate::types::TonTransaction) -> Self {
        Self::TonTransaction(Box::new(val))
    }

}

/// Converts a [`crate::types::TonTransaction`] into [`TonTransaction`].
impl From<crate::types::TonTransaction> for TonTransaction {
    fn from(val: crate::types::TonTransaction) -> Self {
        Self::TonTransaction(Box::new(val))
    }
}

/// TDLib `TonTransactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TonTransactions {
    /// Represents a list of TON Gram transactions
    #[serde(rename(serialize = "tonTransactions", deserialize = "tonTransactions"))]
    TonTransactions(Box<crate::types::TonTransactions>),
}

impl TonTransactions {
    /// Convenience constructor to create a [`TonTransactions::TonTransactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ton_transactions(val: crate::types::TonTransactions) -> Self {
        Self::TonTransactions(Box::new(val))
    }

}

/// Converts a [`crate::types::TonTransactions`] into [`TonTransactions`].
impl From<crate::types::TonTransactions> for TonTransactions {
    fn from(val: crate::types::TonTransactions) -> Self {
        Self::TonTransactions(Box::new(val))
    }
}

/// TDLib `LinkPreviewOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewOptions {
    /// Options to be used for generation of a link preview
    #[serde(rename(serialize = "linkPreviewOptions", deserialize = "linkPreviewOptions"))]
    LinkPreviewOptions(Box<crate::types::LinkPreviewOptions>),
}

impl LinkPreviewOptions {
    /// Convenience constructor to create a [`LinkPreviewOptions::LinkPreviewOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link_preview_options(val: crate::types::LinkPreviewOptions) -> Self {
        Self::LinkPreviewOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::LinkPreviewOptions`] into [`LinkPreviewOptions`].
impl From<crate::types::LinkPreviewOptions> for LinkPreviewOptions {
    fn from(val: crate::types::LinkPreviewOptions) -> Self {
        Self::LinkPreviewOptions(Box::new(val))
    }
}

/// TDLib `AccentColor` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccentColor {
    /// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
    #[serde(rename(serialize = "accentColor", deserialize = "accentColor"))]
    AccentColor(Box<crate::types::AccentColor>),
}

impl AccentColor {
    /// Convenience constructor to create a [`AccentColor::AccentColor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn accent_color(val: crate::types::AccentColor) -> Self {
        Self::AccentColor(Box::new(val))
    }

}

/// Converts a [`crate::types::AccentColor`] into [`AccentColor`].
impl From<crate::types::AccentColor> for AccentColor {
    fn from(val: crate::types::AccentColor) -> Self {
        Self::AccentColor(Box::new(val))
    }
}

/// TDLib `CommunityId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityId {
    /// Contains identifier of a community
    #[serde(rename(serialize = "communityId", deserialize = "communityId"))]
    CommunityId(Box<crate::types::CommunityId>),
}

impl CommunityId {
    /// Convenience constructor to create a [`CommunityId::CommunityId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_id(val: crate::types::CommunityId) -> Self {
        Self::CommunityId(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityId`] into [`CommunityId`].
impl From<crate::types::CommunityId> for CommunityId {
    fn from(val: crate::types::CommunityId) -> Self {
        Self::CommunityId(Box::new(val))
    }
}

/// TDLib `CommunityPermissions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityPermissions {
    /// Describes actions that a user is allowed to take in a community
    #[serde(rename(serialize = "communityPermissions", deserialize = "communityPermissions"))]
    CommunityPermissions(Box<crate::types::CommunityPermissions>),
}

impl CommunityPermissions {
    /// Convenience constructor to create a [`CommunityPermissions::CommunityPermissions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_permissions(val: crate::types::CommunityPermissions) -> Self {
        Self::CommunityPermissions(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityPermissions`] into [`CommunityPermissions`].
impl From<crate::types::CommunityPermissions> for CommunityPermissions {
    fn from(val: crate::types::CommunityPermissions) -> Self {
        Self::CommunityPermissions(Box::new(val))
    }
}

/// TDLib `CommunityAdministratorRights` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityAdministratorRights {
    /// Describes rights of the administrator in a community
    #[serde(rename(serialize = "communityAdministratorRights", deserialize = "communityAdministratorRights"))]
    CommunityAdministratorRights(Box<crate::types::CommunityAdministratorRights>),
}

impl CommunityAdministratorRights {
    /// Convenience constructor to create a [`CommunityAdministratorRights::CommunityAdministratorRights`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_administrator_rights(val: crate::types::CommunityAdministratorRights) -> Self {
        Self::CommunityAdministratorRights(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityAdministratorRights`] into [`CommunityAdministratorRights`].
impl From<crate::types::CommunityAdministratorRights> for CommunityAdministratorRights {
    fn from(val: crate::types::CommunityAdministratorRights) -> Self {
        Self::CommunityAdministratorRights(Box::new(val))
    }
}

/// Provides information about the status of a member in a community
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityMemberStatus {
    /// The user is the owner of the community and has all the administrator privileges
    #[serde(rename(serialize = "communityMemberStatusCreator", deserialize = "communityMemberStatusCreator"))]
    Creator,
    /// The user is a member of the community and has some additional privileges
    #[serde(rename(serialize = "communityMemberStatusAdministrator", deserialize = "communityMemberStatusAdministrator"))]
    Administrator(Box<crate::types::CommunityMemberStatusAdministrator>),
    /// The user is a member of the community, without any additional privileges or restrictions
    #[serde(rename(serialize = "communityMemberStatusMember", deserialize = "communityMemberStatusMember"))]
    Member,
    /// The user or the chat is not a community member
    #[serde(rename(serialize = "communityMemberStatusLeft", deserialize = "communityMemberStatusLeft"))]
    Left,
    /// The user or the chat was banned in the community; implies ban in all chats in the community
    #[serde(rename(serialize = "communityMemberStatusBanned", deserialize = "communityMemberStatusBanned"))]
    Banned,
}

impl CommunityMemberStatus {
    /// Convenience constructor to create a [`CommunityMemberStatus::Administrator`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn administrator(val: crate::types::CommunityMemberStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityMemberStatusAdministrator`] into [`CommunityMemberStatus`].
impl From<crate::types::CommunityMemberStatusAdministrator> for CommunityMemberStatus {
    fn from(val: crate::types::CommunityMemberStatusAdministrator) -> Self {
        Self::Administrator(Box::new(val))
    }
}

/// TDLib `Community` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Community {
    /// Represents a community consisting of supergroup chats, channel chats and chats with bots
    #[serde(rename(serialize = "community", deserialize = "community"))]
    Community(Box<crate::types::Community>),
}

impl Community {
    /// Convenience constructor to create a [`Community::Community`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community(val: crate::types::Community) -> Self {
        Self::Community(Box::new(val))
    }

}

/// Converts a [`crate::types::Community`] into [`Community`].
impl From<crate::types::Community> for Community {
    fn from(val: crate::types::Community) -> Self {
        Self::Community(Box::new(val))
    }
}

/// TDLib `CommunityFullInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CommunityFullInfo {
    /// Contains full information about a community
    #[serde(rename(serialize = "communityFullInfo", deserialize = "communityFullInfo"))]
    CommunityFullInfo(Box<crate::types::CommunityFullInfo>),
}

impl CommunityFullInfo {
    /// Convenience constructor to create a [`CommunityFullInfo::CommunityFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_full_info(val: crate::types::CommunityFullInfo) -> Self {
        Self::CommunityFullInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::CommunityFullInfo`] into [`CommunityFullInfo`].
impl From<crate::types::CommunityFullInfo> for CommunityFullInfo {
    fn from(val: crate::types::CommunityFullInfo) -> Self {
        Self::CommunityFullInfo(Box::new(val))
    }
}

/// TDLib `RestrictionInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RestrictionInfo {
    /// Contains information about restrictions that must be applied to a chat or a message
    #[serde(rename(serialize = "restrictionInfo", deserialize = "restrictionInfo"))]
    RestrictionInfo(Box<crate::types::RestrictionInfo>),
}

impl RestrictionInfo {
    /// Convenience constructor to create a [`RestrictionInfo::RestrictionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn restriction_info(val: crate::types::RestrictionInfo) -> Self {
        Self::RestrictionInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::RestrictionInfo`] into [`RestrictionInfo`].
impl From<crate::types::RestrictionInfo> for RestrictionInfo {
    fn from(val: crate::types::RestrictionInfo) -> Self {
        Self::RestrictionInfo(Box::new(val))
    }
}

/// TDLib `BasicGroup` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BasicGroup {
    /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
    #[serde(rename(serialize = "basicGroup", deserialize = "basicGroup"))]
    BasicGroup(Box<crate::types::BasicGroup>),
}

impl BasicGroup {
    /// Convenience constructor to create a [`BasicGroup::BasicGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn basic_group(val: crate::types::BasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }

}

/// Converts a [`crate::types::BasicGroup`] into [`BasicGroup`].
impl From<crate::types::BasicGroup> for BasicGroup {
    fn from(val: crate::types::BasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }
}

/// TDLib `BasicGroupFullInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BasicGroupFullInfo {
    /// Contains full information about a basic group
    #[serde(rename(serialize = "basicGroupFullInfo", deserialize = "basicGroupFullInfo"))]
    BasicGroupFullInfo(Box<crate::types::BasicGroupFullInfo>),
}

impl BasicGroupFullInfo {
    /// Convenience constructor to create a [`BasicGroupFullInfo::BasicGroupFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn basic_group_full_info(val: crate::types::BasicGroupFullInfo) -> Self {
        Self::BasicGroupFullInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BasicGroupFullInfo`] into [`BasicGroupFullInfo`].
impl From<crate::types::BasicGroupFullInfo> for BasicGroupFullInfo {
    fn from(val: crate::types::BasicGroupFullInfo) -> Self {
        Self::BasicGroupFullInfo(Box::new(val))
    }
}

/// TDLib `PublicPostSearchLimits` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicPostSearchLimits {
    /// Contains information about public post search limits
    #[serde(rename(serialize = "publicPostSearchLimits", deserialize = "publicPostSearchLimits"))]
    PublicPostSearchLimits(Box<crate::types::PublicPostSearchLimits>),
}

impl PublicPostSearchLimits {
    /// Convenience constructor to create a [`PublicPostSearchLimits::PublicPostSearchLimits`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn public_post_search_limits(val: crate::types::PublicPostSearchLimits) -> Self {
        Self::PublicPostSearchLimits(Box::new(val))
    }

}

/// Converts a [`crate::types::PublicPostSearchLimits`] into [`PublicPostSearchLimits`].
impl From<crate::types::PublicPostSearchLimits> for PublicPostSearchLimits {
    fn from(val: crate::types::PublicPostSearchLimits) -> Self {
        Self::PublicPostSearchLimits(Box::new(val))
    }
}

/// TDLib `ForwardSource` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ForwardSource {
    /// Contains information about the last message from which a new message was forwarded last time
    #[serde(rename(serialize = "forwardSource", deserialize = "forwardSource"))]
    ForwardSource(Box<crate::types::ForwardSource>),
}

impl ForwardSource {
    /// Convenience constructor to create a [`ForwardSource::ForwardSource`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forward_source(val: crate::types::ForwardSource) -> Self {
        Self::ForwardSource(Box::new(val))
    }

}

/// Converts a [`crate::types::ForwardSource`] into [`ForwardSource`].
impl From<crate::types::ForwardSource> for ForwardSource {
    fn from(val: crate::types::ForwardSource) -> Self {
        Self::ForwardSource(Box::new(val))
    }
}

/// TDLib `PaidReactor` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidReactor {
    /// Contains information about a user who added paid reactions
    #[serde(rename(serialize = "paidReactor", deserialize = "paidReactor"))]
    PaidReactor(Box<crate::types::PaidReactor>),
}

impl PaidReactor {
    /// Convenience constructor to create a [`PaidReactor::PaidReactor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_reactor(val: crate::types::PaidReactor) -> Self {
        Self::PaidReactor(Box::new(val))
    }

}

/// Converts a [`crate::types::PaidReactor`] into [`PaidReactor`].
impl From<crate::types::PaidReactor> for PaidReactor {
    fn from(val: crate::types::PaidReactor) -> Self {
        Self::PaidReactor(Box::new(val))
    }
}

/// TDLib `FactCheck` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FactCheck {
    /// Describes a fact-check added to the message by an independent checker
    #[serde(rename(serialize = "factCheck", deserialize = "factCheck"))]
    FactCheck(Box<crate::types::FactCheck>),
}

impl FactCheck {
    /// Convenience constructor to create a [`FactCheck::FactCheck`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fact_check(val: crate::types::FactCheck) -> Self {
        Self::FactCheck(Box::new(val))
    }

}

/// Converts a [`crate::types::FactCheck`] into [`FactCheck`].
impl From<crate::types::FactCheck> for FactCheck {
    fn from(val: crate::types::FactCheck) -> Self {
        Self::FactCheck(Box::new(val))
    }
}

/// TDLib `FoundPublicPosts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPublicPosts {
    /// Contains a list of messages found by a public post search
    #[serde(rename(serialize = "foundPublicPosts", deserialize = "foundPublicPosts"))]
    FoundPublicPosts(Box<crate::types::FoundPublicPosts>),
}

impl FoundPublicPosts {
    /// Convenience constructor to create a [`FoundPublicPosts::FoundPublicPosts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_public_posts(val: crate::types::FoundPublicPosts) -> Self {
        Self::FoundPublicPosts(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundPublicPosts`] into [`FoundPublicPosts`].
impl From<crate::types::FoundPublicPosts> for FoundPublicPosts {
    fn from(val: crate::types::FoundPublicPosts) -> Self {
        Self::FoundPublicPosts(Box::new(val))
    }
}

/// TDLib `AdvertisementSponsor` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AdvertisementSponsor {
    /// Information about the sponsor of an advertisement
    #[serde(rename(serialize = "advertisementSponsor", deserialize = "advertisementSponsor"))]
    AdvertisementSponsor(Box<crate::types::AdvertisementSponsor>),
}

impl AdvertisementSponsor {
    /// Convenience constructor to create a [`AdvertisementSponsor::AdvertisementSponsor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn advertisement_sponsor(val: crate::types::AdvertisementSponsor) -> Self {
        Self::AdvertisementSponsor(Box::new(val))
    }

}

/// Converts a [`crate::types::AdvertisementSponsor`] into [`AdvertisementSponsor`].
impl From<crate::types::AdvertisementSponsor> for AdvertisementSponsor {
    fn from(val: crate::types::AdvertisementSponsor) -> Self {
        Self::AdvertisementSponsor(Box::new(val))
    }
}

/// TDLib `ReportOption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportOption {
    /// Describes an option to report an entity to Telegram
    #[serde(rename(serialize = "reportOption", deserialize = "reportOption"))]
    ReportOption(Box<crate::types::ReportOption>),
}

impl ReportOption {
    /// Convenience constructor to create a [`ReportOption::ReportOption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn report_option(val: crate::types::ReportOption) -> Self {
        Self::ReportOption(Box::new(val))
    }

}

/// Converts a [`crate::types::ReportOption`] into [`ReportOption`].
impl From<crate::types::ReportOption> for ReportOption {
    fn from(val: crate::types::ReportOption) -> Self {
        Self::ReportOption(Box::new(val))
    }
}

/// Describes result of sponsored message or chat report
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportSponsoredResult {
    /// The message was reported successfully
    #[serde(rename(serialize = "reportSponsoredResultOk", deserialize = "reportSponsoredResultOk"))]
    Ok,
    /// The sponsored message is too old or not found
    #[serde(rename(serialize = "reportSponsoredResultFailed", deserialize = "reportSponsoredResultFailed"))]
    Failed,
    /// The user must choose an option to report the message and repeat request with the chosen option
    #[serde(rename(serialize = "reportSponsoredResultOptionRequired", deserialize = "reportSponsoredResultOptionRequired"))]
    OptionRequired(Box<crate::types::ReportSponsoredResultOptionRequired>),
    /// Sponsored messages were hidden for the user in all chats
    #[serde(rename(serialize = "reportSponsoredResultAdsHidden", deserialize = "reportSponsoredResultAdsHidden"))]
    AdsHidden,
    /// The user asked to hide sponsored messages, but Telegram Premium is required for this
    #[serde(rename(serialize = "reportSponsoredResultPremiumRequired", deserialize = "reportSponsoredResultPremiumRequired"))]
    PremiumRequired,
}

impl ReportSponsoredResult {
    /// Convenience constructor to create a [`ReportSponsoredResult::OptionRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn option_required(val: crate::types::ReportSponsoredResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }

}

/// Converts a [`crate::types::ReportSponsoredResultOptionRequired`] into [`ReportSponsoredResult`].
impl From<crate::types::ReportSponsoredResultOptionRequired> for ReportSponsoredResult {
    fn from(val: crate::types::ReportSponsoredResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }
}

/// TDLib `FailedToAddMember` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FailedToAddMember {
    /// Contains information about a user who has failed to be added to a chat
    #[serde(rename(serialize = "failedToAddMember", deserialize = "failedToAddMember"))]
    FailedToAddMember(Box<crate::types::FailedToAddMember>),
}

impl FailedToAddMember {
    /// Convenience constructor to create a [`FailedToAddMember::FailedToAddMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn failed_to_add_member(val: crate::types::FailedToAddMember) -> Self {
        Self::FailedToAddMember(Box::new(val))
    }

}

/// Converts a [`crate::types::FailedToAddMember`] into [`FailedToAddMember`].
impl From<crate::types::FailedToAddMember> for FailedToAddMember {
    fn from(val: crate::types::FailedToAddMember) -> Self {
        Self::FailedToAddMember(Box::new(val))
    }
}

/// TDLib `FailedToAddMembers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FailedToAddMembers {
    /// Represents a list of users that has failed to be added to a chat
    #[serde(rename(serialize = "failedToAddMembers", deserialize = "failedToAddMembers"))]
    FailedToAddMembers(Box<crate::types::FailedToAddMembers>),
}

impl FailedToAddMembers {
    /// Convenience constructor to create a [`FailedToAddMembers::FailedToAddMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn failed_to_add_members(val: crate::types::FailedToAddMembers) -> Self {
        Self::FailedToAddMembers(Box::new(val))
    }

}

/// Converts a [`crate::types::FailedToAddMembers`] into [`FailedToAddMembers`].
impl From<crate::types::FailedToAddMembers> for FailedToAddMembers {
    fn from(val: crate::types::FailedToAddMembers) -> Self {
        Self::FailedToAddMembers(Box::new(val))
    }
}

/// TDLib `AccountInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccountInfo {
    /// Contains basic information about another user who started a chat with the current user
    #[serde(rename(serialize = "accountInfo", deserialize = "accountInfo"))]
    AccountInfo(Box<crate::types::AccountInfo>),
}

impl AccountInfo {
    /// Convenience constructor to create a [`AccountInfo::AccountInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn account_info(val: crate::types::AccountInfo) -> Self {
        Self::AccountInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::AccountInfo`] into [`AccountInfo`].
impl From<crate::types::AccountInfo> for AccountInfo {
    fn from(val: crate::types::AccountInfo) -> Self {
        Self::AccountInfo(Box::new(val))
    }
}

/// Describes style of a button
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ButtonStyle {
    /// The button has default style
    #[serde(rename(serialize = "buttonStyleDefault", deserialize = "buttonStyleDefault"))]
    Default,
    /// The button has dark blue color
    #[serde(rename(serialize = "buttonStylePrimary", deserialize = "buttonStylePrimary"))]
    Primary,
    /// The button has red color
    #[serde(rename(serialize = "buttonStyleDanger", deserialize = "buttonStyleDanger"))]
    Danger,
    /// The button has green color
    #[serde(rename(serialize = "buttonStyleSuccess", deserialize = "buttonStyleSuccess"))]
    Success,
    /// The button must be shown as a link. The style is allowed only for callback buttons in inlineButton
    #[serde(rename(serialize = "buttonStyleLink", deserialize = "buttonStyleLink"))]
    Link,
}

/// Describes a keyboard button type
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButtonType {
    /// A simple button, with text that must be sent when the button is pressed
    #[serde(rename(serialize = "keyboardButtonTypeText", deserialize = "keyboardButtonTypeText"))]
    Text,
    /// A button that sends the user's phone number when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestPhoneNumber", deserialize = "keyboardButtonTypeRequestPhoneNumber"))]
    RequestPhoneNumber,
    /// A button that sends the user's location when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestLocation", deserialize = "keyboardButtonTypeRequestLocation"))]
    RequestLocation,
    /// A button that allows the user to create and send a poll when pressed; available only in private chats
    #[serde(rename(serialize = "keyboardButtonTypeRequestPoll", deserialize = "keyboardButtonTypeRequestPoll"))]
    RequestPoll(Box<crate::types::KeyboardButtonTypeRequestPoll>),
    /// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
    #[serde(rename(serialize = "keyboardButtonTypeRequestUsers", deserialize = "keyboardButtonTypeRequestUsers"))]
    RequestUsers(Box<crate::types::KeyboardButtonTypeRequestUsers>),
    /// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
    #[serde(rename(serialize = "keyboardButtonTypeRequestChat", deserialize = "keyboardButtonTypeRequestChat"))]
    RequestChat(Box<crate::types::KeyboardButtonTypeRequestChat>),
    /// A button that requests creation of a managed bot by the current user; available only in private chats. Use the method createBot to complete the request
    #[serde(rename(serialize = "keyboardButtonTypeRequestManagedBot", deserialize = "keyboardButtonTypeRequestManagedBot"))]
    RequestManagedBot(Box<crate::types::KeyboardButtonTypeRequestManagedBot>),
    /// A button that opens a Web App by calling getWebAppUrl
    #[serde(rename(serialize = "keyboardButtonTypeWebApp", deserialize = "keyboardButtonTypeWebApp"))]
    WebApp(Box<crate::types::KeyboardButtonTypeWebApp>),
}

impl KeyboardButtonType {
    /// Convenience constructor to create a [`KeyboardButtonType::RequestPoll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_poll(val: crate::types::KeyboardButtonTypeRequestPoll) -> Self {
        Self::RequestPoll(Box::new(val))
    }

    /// Convenience constructor to create a [`KeyboardButtonType::RequestUsers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_users(val: crate::types::KeyboardButtonTypeRequestUsers) -> Self {
        Self::RequestUsers(Box::new(val))
    }

    /// Convenience constructor to create a [`KeyboardButtonType::RequestChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_chat(val: crate::types::KeyboardButtonTypeRequestChat) -> Self {
        Self::RequestChat(Box::new(val))
    }

    /// Convenience constructor to create a [`KeyboardButtonType::RequestManagedBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_managed_bot(val: crate::types::KeyboardButtonTypeRequestManagedBot) -> Self {
        Self::RequestManagedBot(Box::new(val))
    }

    /// Convenience constructor to create a [`KeyboardButtonType::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::KeyboardButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::KeyboardButtonTypeRequestPoll`] into [`KeyboardButtonType`].
impl From<crate::types::KeyboardButtonTypeRequestPoll> for KeyboardButtonType {
    fn from(val: crate::types::KeyboardButtonTypeRequestPoll) -> Self {
        Self::RequestPoll(Box::new(val))
    }
}

/// Converts a [`crate::types::KeyboardButtonTypeRequestUsers`] into [`KeyboardButtonType`].
impl From<crate::types::KeyboardButtonTypeRequestUsers> for KeyboardButtonType {
    fn from(val: crate::types::KeyboardButtonTypeRequestUsers) -> Self {
        Self::RequestUsers(Box::new(val))
    }
}

/// Converts a [`crate::types::KeyboardButtonTypeRequestChat`] into [`KeyboardButtonType`].
impl From<crate::types::KeyboardButtonTypeRequestChat> for KeyboardButtonType {
    fn from(val: crate::types::KeyboardButtonTypeRequestChat) -> Self {
        Self::RequestChat(Box::new(val))
    }
}

/// Converts a [`crate::types::KeyboardButtonTypeRequestManagedBot`] into [`KeyboardButtonType`].
impl From<crate::types::KeyboardButtonTypeRequestManagedBot> for KeyboardButtonType {
    fn from(val: crate::types::KeyboardButtonTypeRequestManagedBot) -> Self {
        Self::RequestManagedBot(Box::new(val))
    }
}

/// Converts a [`crate::types::KeyboardButtonTypeWebApp`] into [`KeyboardButtonType`].
impl From<crate::types::KeyboardButtonTypeWebApp> for KeyboardButtonType {
    fn from(val: crate::types::KeyboardButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// TDLib `KeyboardButton` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButton {
    /// Represents a single button in a bot keyboard
    #[serde(rename(serialize = "keyboardButton", deserialize = "keyboardButton"))]
    KeyboardButton(Box<crate::types::KeyboardButton>),
}

impl KeyboardButton {
    /// Convenience constructor to create a [`KeyboardButton::KeyboardButton`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn keyboard_button(val: crate::types::KeyboardButton) -> Self {
        Self::KeyboardButton(Box::new(val))
    }

}

/// Converts a [`crate::types::KeyboardButton`] into [`KeyboardButton`].
impl From<crate::types::KeyboardButton> for KeyboardButton {
    fn from(val: crate::types::KeyboardButton) -> Self {
        Self::KeyboardButton(Box::new(val))
    }
}

/// Describes the type of inline keyboard button
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineKeyboardButtonType {
    /// A button that opens a specified URL
    #[serde(rename(serialize = "inlineKeyboardButtonTypeUrl", deserialize = "inlineKeyboardButtonTypeUrl"))]
    Url(Box<crate::types::InlineKeyboardButtonTypeUrl>),
    /// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo; not supported in ephemeral messages
    #[serde(rename(serialize = "inlineKeyboardButtonTypeLoginUrl", deserialize = "inlineKeyboardButtonTypeLoginUrl"))]
    LoginUrl(Box<crate::types::InlineKeyboardButtonTypeLoginUrl>),
    /// A button that opens a Web App by calling openWebApp
    #[serde(rename(serialize = "inlineKeyboardButtonTypeWebApp", deserialize = "inlineKeyboardButtonTypeWebApp"))]
    WebApp(Box<crate::types::InlineKeyboardButtonTypeWebApp>),
    /// A button that sends a callback query to a bot
    #[serde(rename(serialize = "inlineKeyboardButtonTypeCallback", deserialize = "inlineKeyboardButtonTypeCallback"))]
    Callback(Box<crate::types::InlineKeyboardButtonTypeCallback>),
    /// A button that asks for the 2-step verification password of the current user and then sends a callback query to a bot
    #[serde(rename(serialize = "inlineKeyboardButtonTypeCallbackWithPassword", deserialize = "inlineKeyboardButtonTypeCallbackWithPassword"))]
    CallbackWithPassword(Box<crate::types::InlineKeyboardButtonTypeCallbackWithPassword>),
    /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
    #[serde(rename(serialize = "inlineKeyboardButtonTypeCallbackGame", deserialize = "inlineKeyboardButtonTypeCallbackGame"))]
    CallbackGame,
    /// A button that forces an inline query to the bot to be inserted in the input field
    #[serde(rename(serialize = "inlineKeyboardButtonTypeSwitchInline", deserialize = "inlineKeyboardButtonTypeSwitchInline"))]
    SwitchInline(Box<crate::types::InlineKeyboardButtonTypeSwitchInline>),
    /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
    #[serde(rename(serialize = "inlineKeyboardButtonTypeBuy", deserialize = "inlineKeyboardButtonTypeBuy"))]
    Buy,
    /// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
    #[serde(rename(serialize = "inlineKeyboardButtonTypeUser", deserialize = "inlineKeyboardButtonTypeUser"))]
    User(Box<crate::types::InlineKeyboardButtonTypeUser>),
    /// A button that copies specified text to clipboard
    #[serde(rename(serialize = "inlineKeyboardButtonTypeCopyText", deserialize = "inlineKeyboardButtonTypeCopyText"))]
    CopyText(Box<crate::types::InlineKeyboardButtonTypeCopyText>),
    /// A disabled button
    #[serde(rename(serialize = "inlineKeyboardButtonTypeDisabled", deserialize = "inlineKeyboardButtonTypeDisabled"))]
    Disabled,
}

impl InlineKeyboardButtonType {
    /// Convenience constructor to create a [`InlineKeyboardButtonType::Url`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn url(val: crate::types::InlineKeyboardButtonTypeUrl) -> Self {
        Self::Url(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::LoginUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn login_url(val: crate::types::InlineKeyboardButtonTypeLoginUrl) -> Self {
        Self::LoginUrl(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::InlineKeyboardButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::Callback`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn callback(val: crate::types::InlineKeyboardButtonTypeCallback) -> Self {
        Self::Callback(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::CallbackWithPassword`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn callback_with_password(val: crate::types::InlineKeyboardButtonTypeCallbackWithPassword) -> Self {
        Self::CallbackWithPassword(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::SwitchInline`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn switch_inline(val: crate::types::InlineKeyboardButtonTypeSwitchInline) -> Self {
        Self::SwitchInline(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::InlineKeyboardButtonTypeUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineKeyboardButtonType::CopyText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn copy_text(val: crate::types::InlineKeyboardButtonTypeCopyText) -> Self {
        Self::CopyText(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineKeyboardButtonTypeUrl`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeUrl> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeUrl) -> Self {
        Self::Url(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeLoginUrl`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeLoginUrl> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeLoginUrl) -> Self {
        Self::LoginUrl(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeWebApp`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeWebApp> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeCallback`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeCallback> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeCallback) -> Self {
        Self::Callback(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeCallbackWithPassword`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeCallbackWithPassword> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeCallbackWithPassword) -> Self {
        Self::CallbackWithPassword(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeSwitchInline`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeSwitchInline> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeSwitchInline) -> Self {
        Self::SwitchInline(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeUser`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeUser> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineKeyboardButtonTypeCopyText`] into [`InlineKeyboardButtonType`].
impl From<crate::types::InlineKeyboardButtonTypeCopyText> for InlineKeyboardButtonType {
    fn from(val: crate::types::InlineKeyboardButtonTypeCopyText) -> Self {
        Self::CopyText(Box::new(val))
    }
}

/// Describes source of a keyboard button
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButtonSource {
    /// The button is from a bot's message
    #[serde(rename(serialize = "keyboardButtonSourceMessage", deserialize = "keyboardButtonSourceMessage"))]
    Message(Box<crate::types::KeyboardButtonSourceMessage>),
    /// The button is a prepared keyboard button from a Mini App received via getPreparedKeyboardButton
    #[serde(rename(serialize = "keyboardButtonSourceWebApp", deserialize = "keyboardButtonSourceWebApp"))]
    WebApp(Box<crate::types::KeyboardButtonSourceWebApp>),
}

impl KeyboardButtonSource {
    /// Convenience constructor to create a [`KeyboardButtonSource::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::KeyboardButtonSourceMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`KeyboardButtonSource::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::KeyboardButtonSourceWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::KeyboardButtonSourceMessage`] into [`KeyboardButtonSource`].
impl From<crate::types::KeyboardButtonSourceMessage> for KeyboardButtonSource {
    fn from(val: crate::types::KeyboardButtonSourceMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::KeyboardButtonSourceWebApp`] into [`KeyboardButtonSource`].
impl From<crate::types::KeyboardButtonSourceWebApp> for KeyboardButtonSource {
    fn from(val: crate::types::KeyboardButtonSourceWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// TDLib `InlineKeyboardButton` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineKeyboardButton {
    /// Represents a single button in an inline keyboard
    #[serde(rename(serialize = "inlineKeyboardButton", deserialize = "inlineKeyboardButton"))]
    InlineKeyboardButton(Box<crate::types::InlineKeyboardButton>),
}

impl InlineKeyboardButton {
    /// Convenience constructor to create a [`InlineKeyboardButton::InlineKeyboardButton`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_keyboard_button(val: crate::types::InlineKeyboardButton) -> Self {
        Self::InlineKeyboardButton(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineKeyboardButton`] into [`InlineKeyboardButton`].
impl From<crate::types::InlineKeyboardButton> for InlineKeyboardButton {
    fn from(val: crate::types::InlineKeyboardButton) -> Self {
        Self::InlineKeyboardButton(Box::new(val))
    }
}

/// Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReplyMarkup {
    /// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, updateChatReplyMarkup with reply_markup_message == null will be sent
    #[serde(rename(serialize = "replyMarkupRemoveKeyboard", deserialize = "replyMarkupRemoveKeyboard"))]
    RemoveKeyboard(Box<crate::types::ReplyMarkupRemoveKeyboard>),
    /// Instructs application to force a reply to this message
    #[serde(rename(serialize = "replyMarkupForceReply", deserialize = "replyMarkupForceReply"))]
    ForceReply(Box<crate::types::ReplyMarkupForceReply>),
    /// Contains a custom keyboard layout to quickly reply to bots
    #[serde(rename(serialize = "replyMarkupShowKeyboard", deserialize = "replyMarkupShowKeyboard"))]
    ShowKeyboard(Box<crate::types::ReplyMarkupShowKeyboard>),
    /// Contains an inline keyboard layout
    #[serde(rename(serialize = "replyMarkupInlineKeyboard", deserialize = "replyMarkupInlineKeyboard"))]
    InlineKeyboard(Box<crate::types::ReplyMarkupInlineKeyboard>),
}

impl ReplyMarkup {
    /// Convenience constructor to create a [`ReplyMarkup::RemoveKeyboard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn remove_keyboard(val: crate::types::ReplyMarkupRemoveKeyboard) -> Self {
        Self::RemoveKeyboard(Box::new(val))
    }

    /// Convenience constructor to create a [`ReplyMarkup::ForceReply`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn force_reply(val: crate::types::ReplyMarkupForceReply) -> Self {
        Self::ForceReply(Box::new(val))
    }

    /// Convenience constructor to create a [`ReplyMarkup::ShowKeyboard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn show_keyboard(val: crate::types::ReplyMarkupShowKeyboard) -> Self {
        Self::ShowKeyboard(Box::new(val))
    }

    /// Convenience constructor to create a [`ReplyMarkup::InlineKeyboard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_keyboard(val: crate::types::ReplyMarkupInlineKeyboard) -> Self {
        Self::InlineKeyboard(Box::new(val))
    }

}

/// Converts a [`crate::types::ReplyMarkupRemoveKeyboard`] into [`ReplyMarkup`].
impl From<crate::types::ReplyMarkupRemoveKeyboard> for ReplyMarkup {
    fn from(val: crate::types::ReplyMarkupRemoveKeyboard) -> Self {
        Self::RemoveKeyboard(Box::new(val))
    }
}

/// Converts a [`crate::types::ReplyMarkupForceReply`] into [`ReplyMarkup`].
impl From<crate::types::ReplyMarkupForceReply> for ReplyMarkup {
    fn from(val: crate::types::ReplyMarkupForceReply) -> Self {
        Self::ForceReply(Box::new(val))
    }
}

/// Converts a [`crate::types::ReplyMarkupShowKeyboard`] into [`ReplyMarkup`].
impl From<crate::types::ReplyMarkupShowKeyboard> for ReplyMarkup {
    fn from(val: crate::types::ReplyMarkupShowKeyboard) -> Self {
        Self::ShowKeyboard(Box::new(val))
    }
}

/// Converts a [`crate::types::ReplyMarkupInlineKeyboard`] into [`ReplyMarkup`].
impl From<crate::types::ReplyMarkupInlineKeyboard> for ReplyMarkup {
    fn from(val: crate::types::ReplyMarkupInlineKeyboard) -> Self {
        Self::InlineKeyboard(Box::new(val))
    }
}

/// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl or an external link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LoginUrlInfo {
    /// An HTTP URL needs to be open
    #[serde(rename(serialize = "loginUrlInfoOpen", deserialize = "loginUrlInfoOpen"))]
    Open(Box<crate::types::LoginUrlInfoOpen>),
    /// An authorization confirmation dialog needs to be shown to the user
    #[serde(rename(serialize = "loginUrlInfoRequestConfirmation", deserialize = "loginUrlInfoRequestConfirmation"))]
    RequestConfirmation(Box<crate::types::LoginUrlInfoRequestConfirmation>),
}

impl LoginUrlInfo {
    /// Convenience constructor to create a [`LoginUrlInfo::Open`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn open(val: crate::types::LoginUrlInfoOpen) -> Self {
        Self::Open(Box::new(val))
    }

    /// Convenience constructor to create a [`LoginUrlInfo::RequestConfirmation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_confirmation(val: crate::types::LoginUrlInfoRequestConfirmation) -> Self {
        Self::RequestConfirmation(Box::new(val))
    }

}

/// Converts a [`crate::types::LoginUrlInfoOpen`] into [`LoginUrlInfo`].
impl From<crate::types::LoginUrlInfoOpen> for LoginUrlInfo {
    fn from(val: crate::types::LoginUrlInfoOpen) -> Self {
        Self::Open(Box::new(val))
    }
}

/// Converts a [`crate::types::LoginUrlInfoRequestConfirmation`] into [`LoginUrlInfo`].
impl From<crate::types::LoginUrlInfoRequestConfirmation> for LoginUrlInfo {
    fn from(val: crate::types::LoginUrlInfoRequestConfirmation) -> Self {
        Self::RequestConfirmation(Box::new(val))
    }
}

/// TDLib `OauthLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OauthLinkInfo {
    /// Information about the OAuth authorization
    #[serde(rename(serialize = "oauthLinkInfo", deserialize = "oauthLinkInfo"))]
    OauthLinkInfo(Box<crate::types::OauthLinkInfo>),
}

impl OauthLinkInfo {
    /// Convenience constructor to create a [`OauthLinkInfo::OauthLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn oauth_link_info(val: crate::types::OauthLinkInfo) -> Self {
        Self::OauthLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::OauthLinkInfo`] into [`OauthLinkInfo`].
impl From<crate::types::OauthLinkInfo> for OauthLinkInfo {
    fn from(val: crate::types::OauthLinkInfo) -> Self {
        Self::OauthLinkInfo(Box::new(val))
    }
}

/// Describes a built-in theme of an official application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BuiltInTheme {
    /// Classic light theme
    #[serde(rename(serialize = "builtInThemeClassic", deserialize = "builtInThemeClassic"))]
    Classic,
    /// Regular light theme
    #[serde(rename(serialize = "builtInThemeDay", deserialize = "builtInThemeDay"))]
    Day,
    /// Regular dark theme
    #[serde(rename(serialize = "builtInThemeNight", deserialize = "builtInThemeNight"))]
    Night,
    /// Tinted dark theme
    #[serde(rename(serialize = "builtInThemeTinted", deserialize = "builtInThemeTinted"))]
    Tinted,
    /// Arctic light theme
    #[serde(rename(serialize = "builtInThemeArctic", deserialize = "builtInThemeArctic"))]
    Arctic,
}

/// TDLib `ThemeSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ThemeSettings {
    /// Describes theme settings
    #[serde(rename(serialize = "themeSettings", deserialize = "themeSettings"))]
    ThemeSettings(Box<crate::types::ThemeSettings>),
}

impl ThemeSettings {
    /// Convenience constructor to create a [`ThemeSettings::ThemeSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn theme_settings(val: crate::types::ThemeSettings) -> Self {
        Self::ThemeSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ThemeSettings`] into [`ThemeSettings`].
impl From<crate::types::ThemeSettings> for ThemeSettings {
    fn from(val: crate::types::ThemeSettings) -> Self {
        Self::ThemeSettings(Box::new(val))
    }
}

/// TDLib `InlineButton` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineButton {
    /// Represents a button inside a rich message
    #[serde(rename(serialize = "inlineButton", deserialize = "inlineButton"))]
    InlineButton(Box<crate::types::InlineButton>),
}

impl InlineButton {
    /// Convenience constructor to create a [`InlineButton::InlineButton`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_button(val: crate::types::InlineButton) -> Self {
        Self::InlineButton(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineButton`] into [`InlineButton`].
impl From<crate::types::InlineButton> for InlineButton {
    fn from(val: crate::types::InlineButton) -> Self {
        Self::InlineButton(Box::new(val))
    }
}

/// TDLib `PageBlockListItem` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockListItem {
    /// Describes an item of a list page block
    #[serde(rename(serialize = "pageBlockListItem", deserialize = "pageBlockListItem"))]
    PageBlockListItem(Box<crate::types::PageBlockListItem>),
}

impl PageBlockListItem {
    /// Convenience constructor to create a [`PageBlockListItem::PageBlockListItem`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn page_block_list_item(val: crate::types::PageBlockListItem) -> Self {
        Self::PageBlockListItem(Box::new(val))
    }

}

/// Converts a [`crate::types::PageBlockListItem`] into [`PageBlockListItem`].
impl From<crate::types::PageBlockListItem> for PageBlockListItem {
    fn from(val: crate::types::PageBlockListItem) -> Self {
        Self::PageBlockListItem(Box::new(val))
    }
}

/// TDLib `InputPageBlockListItem` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPageBlockListItem {
    /// Describes an item of a list page block to be sent
    #[serde(rename(serialize = "inputPageBlockListItem", deserialize = "inputPageBlockListItem"))]
    InputPageBlockListItem(Box<crate::types::InputPageBlockListItem>),
}

impl InputPageBlockListItem {
    /// Convenience constructor to create a [`InputPageBlockListItem::InputPageBlockListItem`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_page_block_list_item(val: crate::types::InputPageBlockListItem) -> Self {
        Self::InputPageBlockListItem(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPageBlockListItem`] into [`InputPageBlockListItem`].
impl From<crate::types::InputPageBlockListItem> for InputPageBlockListItem {
    fn from(val: crate::types::InputPageBlockListItem) -> Self {
        Self::InputPageBlockListItem(Box::new(val))
    }
}

/// Describes a horizontal alignment of a table cell content
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockHorizontalAlignment {
    /// The content must be left-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentLeft", deserialize = "pageBlockHorizontalAlignmentLeft"))]
    Left,
    /// The content must be center-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentCenter", deserialize = "pageBlockHorizontalAlignmentCenter"))]
    Center,
    /// The content must be right-aligned
    #[serde(rename(serialize = "pageBlockHorizontalAlignmentRight", deserialize = "pageBlockHorizontalAlignmentRight"))]
    Right,
}

/// Describes a Vertical alignment of a table cell content
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockVerticalAlignment {
    /// The content must be top-aligned
    #[serde(rename(serialize = "pageBlockVerticalAlignmentTop", deserialize = "pageBlockVerticalAlignmentTop"))]
    Top,
    /// The content must be middle-aligned
    #[serde(rename(serialize = "pageBlockVerticalAlignmentMiddle", deserialize = "pageBlockVerticalAlignmentMiddle"))]
    Middle,
    /// The content must be bottom-aligned
    #[serde(rename(serialize = "pageBlockVerticalAlignmentBottom", deserialize = "pageBlockVerticalAlignmentBottom"))]
    Bottom,
}

/// TDLib `PageBlockTableCell` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockTableCell {
    /// Represents a cell of a table
    #[serde(rename(serialize = "pageBlockTableCell", deserialize = "pageBlockTableCell"))]
    PageBlockTableCell(Box<crate::types::PageBlockTableCell>),
}

impl PageBlockTableCell {
    /// Convenience constructor to create a [`PageBlockTableCell::PageBlockTableCell`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn page_block_table_cell(val: crate::types::PageBlockTableCell) -> Self {
        Self::PageBlockTableCell(Box::new(val))
    }

}

/// Converts a [`crate::types::PageBlockTableCell`] into [`PageBlockTableCell`].
impl From<crate::types::PageBlockTableCell> for PageBlockTableCell {
    fn from(val: crate::types::PageBlockTableCell) -> Self {
        Self::PageBlockTableCell(Box::new(val))
    }
}

/// TDLib `PageBlockRelatedArticle` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockRelatedArticle {
    /// Contains information about a related article
    #[serde(rename(serialize = "pageBlockRelatedArticle", deserialize = "pageBlockRelatedArticle"))]
    PageBlockRelatedArticle(Box<crate::types::PageBlockRelatedArticle>),
}

impl PageBlockRelatedArticle {
    /// Convenience constructor to create a [`PageBlockRelatedArticle::PageBlockRelatedArticle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn page_block_related_article(val: crate::types::PageBlockRelatedArticle) -> Self {
        Self::PageBlockRelatedArticle(Box::new(val))
    }

}

/// Converts a [`crate::types::PageBlockRelatedArticle`] into [`PageBlockRelatedArticle`].
impl From<crate::types::PageBlockRelatedArticle> for PageBlockRelatedArticle {
    fn from(val: crate::types::PageBlockRelatedArticle) -> Self {
        Self::PageBlockRelatedArticle(Box::new(val))
    }
}

/// Describes a block of an instant view for a web page or a block of a rich message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlock {
    /// The title of a page; instant view only
    #[serde(rename(serialize = "pageBlockTitle", deserialize = "pageBlockTitle"))]
    Title(Box<crate::types::PageBlockTitle>),
    /// The subtitle of a page; instant view only
    #[serde(rename(serialize = "pageBlockSubtitle", deserialize = "pageBlockSubtitle"))]
    Subtitle(Box<crate::types::PageBlockSubtitle>),
    /// The author and publishing date of a page; instant view only
    #[serde(rename(serialize = "pageBlockAuthorDate", deserialize = "pageBlockAuthorDate"))]
    AuthorDate(Box<crate::types::PageBlockAuthorDate>),
    /// A header; instant view only
    #[serde(rename(serialize = "pageBlockHeader", deserialize = "pageBlockHeader"))]
    Header(Box<crate::types::PageBlockHeader>),
    /// A subheader; instant view only
    #[serde(rename(serialize = "pageBlockSubheader", deserialize = "pageBlockSubheader"))]
    Subheader(Box<crate::types::PageBlockSubheader>),
    /// A section heading
    #[serde(rename(serialize = "pageBlockSectionHeading", deserialize = "pageBlockSectionHeading"))]
    SectionHeading(Box<crate::types::PageBlockSectionHeading>),
    /// A kicker; instant view only
    #[serde(rename(serialize = "pageBlockKicker", deserialize = "pageBlockKicker"))]
    Kicker(Box<crate::types::PageBlockKicker>),
    /// A text paragraph
    #[serde(rename(serialize = "pageBlockParagraph", deserialize = "pageBlockParagraph"))]
    Paragraph(Box<crate::types::PageBlockParagraph>),
    /// A preformatted text paragraph
    #[serde(rename(serialize = "pageBlockPreformatted", deserialize = "pageBlockPreformatted"))]
    Preformatted(Box<crate::types::PageBlockPreformatted>),
    /// The footer of a page
    #[serde(rename(serialize = "pageBlockFooter", deserialize = "pageBlockFooter"))]
    Footer(Box<crate::types::PageBlockFooter>),
    /// A "Thinking..." placeholder; for pending rich messages only
    #[serde(rename(serialize = "pageBlockThinking", deserialize = "pageBlockThinking"))]
    Thinking(Box<crate::types::PageBlockThinking>),
    /// An empty block separating a page
    #[serde(rename(serialize = "pageBlockDivider", deserialize = "pageBlockDivider"))]
    Divider,
    /// A mathematical expression
    #[serde(rename(serialize = "pageBlockMathematicalExpression", deserialize = "pageBlockMathematicalExpression"))]
    MathematicalExpression(Box<crate::types::PageBlockMathematicalExpression>),
    /// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
    #[serde(rename(serialize = "pageBlockAnchor", deserialize = "pageBlockAnchor"))]
    Anchor(Box<crate::types::PageBlockAnchor>),
    /// A list of data blocks
    #[serde(rename(serialize = "pageBlockList", deserialize = "pageBlockList"))]
    List(Box<crate::types::PageBlockList>),
    /// A block quote
    #[serde(rename(serialize = "pageBlockBlockQuote", deserialize = "pageBlockBlockQuote"))]
    BlockQuote(Box<crate::types::PageBlockBlockQuote>),
    /// An expandable block quote
    #[serde(rename(serialize = "pageBlockExpandableBlockQuote", deserialize = "pageBlockExpandableBlockQuote"))]
    ExpandableBlockQuote(Box<crate::types::PageBlockExpandableBlockQuote>),
    /// A pull quote
    #[serde(rename(serialize = "pageBlockPullQuote", deserialize = "pageBlockPullQuote"))]
    PullQuote(Box<crate::types::PageBlockPullQuote>),
    /// An animation
    #[serde(rename(serialize = "pageBlockAnimation", deserialize = "pageBlockAnimation"))]
    Animation(Box<crate::types::PageBlockAnimation>),
    /// An audio file
    #[serde(rename(serialize = "pageBlockAudio", deserialize = "pageBlockAudio"))]
    Audio(Box<crate::types::PageBlockAudio>),
    /// A general file
    #[serde(rename(serialize = "pageBlockDocument", deserialize = "pageBlockDocument"))]
    Document(Box<crate::types::PageBlockDocument>),
    /// A photo
    #[serde(rename(serialize = "pageBlockPhoto", deserialize = "pageBlockPhoto"))]
    Photo(Box<crate::types::PageBlockPhoto>),
    /// A video
    #[serde(rename(serialize = "pageBlockVideo", deserialize = "pageBlockVideo"))]
    Video(Box<crate::types::PageBlockVideo>),
    /// A voice note
    #[serde(rename(serialize = "pageBlockVoiceNote", deserialize = "pageBlockVoiceNote"))]
    VoiceNote(Box<crate::types::PageBlockVoiceNote>),
    /// A page cover; instant view only
    #[serde(rename(serialize = "pageBlockCover", deserialize = "pageBlockCover"))]
    Cover(Box<crate::types::PageBlockCover>),
    /// An embedded web page; instant view only
    #[serde(rename(serialize = "pageBlockEmbedded", deserialize = "pageBlockEmbedded"))]
    Embedded(Box<crate::types::PageBlockEmbedded>),
    /// An embedded post; instant view only
    #[serde(rename(serialize = "pageBlockEmbeddedPost", deserialize = "pageBlockEmbeddedPost"))]
    EmbeddedPost(Box<crate::types::PageBlockEmbeddedPost>),
    /// A collage
    #[serde(rename(serialize = "pageBlockCollage", deserialize = "pageBlockCollage"))]
    Collage(Box<crate::types::PageBlockCollage>),
    /// A slideshow
    #[serde(rename(serialize = "pageBlockSlideshow", deserialize = "pageBlockSlideshow"))]
    Slideshow(Box<crate::types::PageBlockSlideshow>),
    /// A link to a chat; instant view only
    #[serde(rename(serialize = "pageBlockChatLink", deserialize = "pageBlockChatLink"))]
    ChatLink(Box<crate::types::PageBlockChatLink>),
    /// A table
    #[serde(rename(serialize = "pageBlockTable", deserialize = "pageBlockTable"))]
    Table(Box<crate::types::PageBlockTable>),
    /// A collapsible block
    #[serde(rename(serialize = "pageBlockDetails", deserialize = "pageBlockDetails"))]
    Details(Box<crate::types::PageBlockDetails>),
    /// Related articles; instant view only
    #[serde(rename(serialize = "pageBlockRelatedArticles", deserialize = "pageBlockRelatedArticles"))]
    RelatedArticles(Box<crate::types::PageBlockRelatedArticles>),
    /// A map
    #[serde(rename(serialize = "pageBlockMap", deserialize = "pageBlockMap"))]
    Map(Box<crate::types::PageBlockMap>),
    /// A list of buttons shown in a row
    #[serde(rename(serialize = "pageBlockButtonRow", deserialize = "pageBlockButtonRow"))]
    ButtonRow(Box<crate::types::PageBlockButtonRow>),
    /// Represents a block unsupported by the current application version
    #[serde(rename(serialize = "pageBlockUnsupported", deserialize = "pageBlockUnsupported"))]
    Unsupported,
}

impl PageBlock {
    /// Convenience constructor to create a [`PageBlock::Title`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn title(val: crate::types::PageBlockTitle) -> Self {
        Self::Title(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Subtitle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn subtitle(val: crate::types::PageBlockSubtitle) -> Self {
        Self::Subtitle(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::AuthorDate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn author_date(val: crate::types::PageBlockAuthorDate) -> Self {
        Self::AuthorDate(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Header`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn header(val: crate::types::PageBlockHeader) -> Self {
        Self::Header(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Subheader`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn subheader(val: crate::types::PageBlockSubheader) -> Self {
        Self::Subheader(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::SectionHeading`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn section_heading(val: crate::types::PageBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Kicker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn kicker(val: crate::types::PageBlockKicker) -> Self {
        Self::Kicker(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Paragraph`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paragraph(val: crate::types::PageBlockParagraph) -> Self {
        Self::Paragraph(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Preformatted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn preformatted(val: crate::types::PageBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Footer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn footer(val: crate::types::PageBlockFooter) -> Self {
        Self::Footer(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Thinking`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn thinking(val: crate::types::PageBlockThinking) -> Self {
        Self::Thinking(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::MathematicalExpression`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mathematical_expression(val: crate::types::PageBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Anchor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn anchor(val: crate::types::PageBlockAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::List`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn list(val: crate::types::PageBlockList) -> Self {
        Self::List(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::BlockQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn block_quote(val: crate::types::PageBlockBlockQuote) -> Self {
        Self::BlockQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::ExpandableBlockQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn expandable_block_quote(val: crate::types::PageBlockExpandableBlockQuote) -> Self {
        Self::ExpandableBlockQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::PullQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pull_quote(val: crate::types::PageBlockPullQuote) -> Self {
        Self::PullQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::PageBlockAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::PageBlockAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::PageBlockDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::PageBlockPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::PageBlockVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::PageBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Cover`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn cover(val: crate::types::PageBlockCover) -> Self {
        Self::Cover(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Embedded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn embedded(val: crate::types::PageBlockEmbedded) -> Self {
        Self::Embedded(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::EmbeddedPost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn embedded_post(val: crate::types::PageBlockEmbeddedPost) -> Self {
        Self::EmbeddedPost(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Collage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn collage(val: crate::types::PageBlockCollage) -> Self {
        Self::Collage(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Slideshow`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn slideshow(val: crate::types::PageBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::ChatLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_link(val: crate::types::PageBlockChatLink) -> Self {
        Self::ChatLink(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Table`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn table(val: crate::types::PageBlockTable) -> Self {
        Self::Table(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Details`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn details(val: crate::types::PageBlockDetails) -> Self {
        Self::Details(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::RelatedArticles`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn related_articles(val: crate::types::PageBlockRelatedArticles) -> Self {
        Self::RelatedArticles(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::Map`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn map(val: crate::types::PageBlockMap) -> Self {
        Self::Map(Box::new(val))
    }

    /// Convenience constructor to create a [`PageBlock::ButtonRow`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn button_row(val: crate::types::PageBlockButtonRow) -> Self {
        Self::ButtonRow(Box::new(val))
    }

}

/// Converts a [`crate::types::PageBlockTitle`] into [`PageBlock`].
impl From<crate::types::PageBlockTitle> for PageBlock {
    fn from(val: crate::types::PageBlockTitle) -> Self {
        Self::Title(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockSubtitle`] into [`PageBlock`].
impl From<crate::types::PageBlockSubtitle> for PageBlock {
    fn from(val: crate::types::PageBlockSubtitle) -> Self {
        Self::Subtitle(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockAuthorDate`] into [`PageBlock`].
impl From<crate::types::PageBlockAuthorDate> for PageBlock {
    fn from(val: crate::types::PageBlockAuthorDate) -> Self {
        Self::AuthorDate(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockHeader`] into [`PageBlock`].
impl From<crate::types::PageBlockHeader> for PageBlock {
    fn from(val: crate::types::PageBlockHeader) -> Self {
        Self::Header(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockSubheader`] into [`PageBlock`].
impl From<crate::types::PageBlockSubheader> for PageBlock {
    fn from(val: crate::types::PageBlockSubheader) -> Self {
        Self::Subheader(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockSectionHeading`] into [`PageBlock`].
impl From<crate::types::PageBlockSectionHeading> for PageBlock {
    fn from(val: crate::types::PageBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockKicker`] into [`PageBlock`].
impl From<crate::types::PageBlockKicker> for PageBlock {
    fn from(val: crate::types::PageBlockKicker) -> Self {
        Self::Kicker(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockParagraph`] into [`PageBlock`].
impl From<crate::types::PageBlockParagraph> for PageBlock {
    fn from(val: crate::types::PageBlockParagraph) -> Self {
        Self::Paragraph(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockPreformatted`] into [`PageBlock`].
impl From<crate::types::PageBlockPreformatted> for PageBlock {
    fn from(val: crate::types::PageBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockFooter`] into [`PageBlock`].
impl From<crate::types::PageBlockFooter> for PageBlock {
    fn from(val: crate::types::PageBlockFooter) -> Self {
        Self::Footer(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockThinking`] into [`PageBlock`].
impl From<crate::types::PageBlockThinking> for PageBlock {
    fn from(val: crate::types::PageBlockThinking) -> Self {
        Self::Thinking(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockMathematicalExpression`] into [`PageBlock`].
impl From<crate::types::PageBlockMathematicalExpression> for PageBlock {
    fn from(val: crate::types::PageBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockAnchor`] into [`PageBlock`].
impl From<crate::types::PageBlockAnchor> for PageBlock {
    fn from(val: crate::types::PageBlockAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockList`] into [`PageBlock`].
impl From<crate::types::PageBlockList> for PageBlock {
    fn from(val: crate::types::PageBlockList) -> Self {
        Self::List(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockBlockQuote`] into [`PageBlock`].
impl From<crate::types::PageBlockBlockQuote> for PageBlock {
    fn from(val: crate::types::PageBlockBlockQuote) -> Self {
        Self::BlockQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockExpandableBlockQuote`] into [`PageBlock`].
impl From<crate::types::PageBlockExpandableBlockQuote> for PageBlock {
    fn from(val: crate::types::PageBlockExpandableBlockQuote) -> Self {
        Self::ExpandableBlockQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockPullQuote`] into [`PageBlock`].
impl From<crate::types::PageBlockPullQuote> for PageBlock {
    fn from(val: crate::types::PageBlockPullQuote) -> Self {
        Self::PullQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockAnimation`] into [`PageBlock`].
impl From<crate::types::PageBlockAnimation> for PageBlock {
    fn from(val: crate::types::PageBlockAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockAudio`] into [`PageBlock`].
impl From<crate::types::PageBlockAudio> for PageBlock {
    fn from(val: crate::types::PageBlockAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockDocument`] into [`PageBlock`].
impl From<crate::types::PageBlockDocument> for PageBlock {
    fn from(val: crate::types::PageBlockDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockPhoto`] into [`PageBlock`].
impl From<crate::types::PageBlockPhoto> for PageBlock {
    fn from(val: crate::types::PageBlockPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockVideo`] into [`PageBlock`].
impl From<crate::types::PageBlockVideo> for PageBlock {
    fn from(val: crate::types::PageBlockVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockVoiceNote`] into [`PageBlock`].
impl From<crate::types::PageBlockVoiceNote> for PageBlock {
    fn from(val: crate::types::PageBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockCover`] into [`PageBlock`].
impl From<crate::types::PageBlockCover> for PageBlock {
    fn from(val: crate::types::PageBlockCover) -> Self {
        Self::Cover(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockEmbedded`] into [`PageBlock`].
impl From<crate::types::PageBlockEmbedded> for PageBlock {
    fn from(val: crate::types::PageBlockEmbedded) -> Self {
        Self::Embedded(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockEmbeddedPost`] into [`PageBlock`].
impl From<crate::types::PageBlockEmbeddedPost> for PageBlock {
    fn from(val: crate::types::PageBlockEmbeddedPost) -> Self {
        Self::EmbeddedPost(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockCollage`] into [`PageBlock`].
impl From<crate::types::PageBlockCollage> for PageBlock {
    fn from(val: crate::types::PageBlockCollage) -> Self {
        Self::Collage(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockSlideshow`] into [`PageBlock`].
impl From<crate::types::PageBlockSlideshow> for PageBlock {
    fn from(val: crate::types::PageBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockChatLink`] into [`PageBlock`].
impl From<crate::types::PageBlockChatLink> for PageBlock {
    fn from(val: crate::types::PageBlockChatLink) -> Self {
        Self::ChatLink(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockTable`] into [`PageBlock`].
impl From<crate::types::PageBlockTable> for PageBlock {
    fn from(val: crate::types::PageBlockTable) -> Self {
        Self::Table(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockDetails`] into [`PageBlock`].
impl From<crate::types::PageBlockDetails> for PageBlock {
    fn from(val: crate::types::PageBlockDetails) -> Self {
        Self::Details(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockRelatedArticles`] into [`PageBlock`].
impl From<crate::types::PageBlockRelatedArticles> for PageBlock {
    fn from(val: crate::types::PageBlockRelatedArticles) -> Self {
        Self::RelatedArticles(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockMap`] into [`PageBlock`].
impl From<crate::types::PageBlockMap> for PageBlock {
    fn from(val: crate::types::PageBlockMap) -> Self {
        Self::Map(Box::new(val))
    }
}

/// Converts a [`crate::types::PageBlockButtonRow`] into [`PageBlock`].
impl From<crate::types::PageBlockButtonRow> for PageBlock {
    fn from(val: crate::types::PageBlockButtonRow) -> Self {
        Self::ButtonRow(Box::new(val))
    }
}

/// TDLib `WebPageInstantView` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebPageInstantView {
    /// Describes an instant view page for a web page
    #[serde(rename(serialize = "webPageInstantView", deserialize = "webPageInstantView"))]
    WebPageInstantView(Box<crate::types::WebPageInstantView>),
}

impl WebPageInstantView {
    /// Convenience constructor to create a [`WebPageInstantView::WebPageInstantView`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_page_instant_view(val: crate::types::WebPageInstantView) -> Self {
        Self::WebPageInstantView(Box::new(val))
    }

}

/// Converts a [`crate::types::WebPageInstantView`] into [`WebPageInstantView`].
impl From<crate::types::WebPageInstantView> for WebPageInstantView {
    fn from(val: crate::types::WebPageInstantView) -> Self {
        Self::WebPageInstantView(Box::new(val))
    }
}

/// Describes a media from a link preview album
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewAlbumMedia {
    /// The media is a photo
    #[serde(rename(serialize = "linkPreviewAlbumMediaPhoto", deserialize = "linkPreviewAlbumMediaPhoto"))]
    Photo(Box<crate::types::LinkPreviewAlbumMediaPhoto>),
    /// The media is a video
    #[serde(rename(serialize = "linkPreviewAlbumMediaVideo", deserialize = "linkPreviewAlbumMediaVideo"))]
    Video(Box<crate::types::LinkPreviewAlbumMediaVideo>),
}

impl LinkPreviewAlbumMedia {
    /// Convenience constructor to create a [`LinkPreviewAlbumMedia::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::LinkPreviewAlbumMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewAlbumMedia::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::LinkPreviewAlbumMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::LinkPreviewAlbumMediaPhoto`] into [`LinkPreviewAlbumMedia`].
impl From<crate::types::LinkPreviewAlbumMediaPhoto> for LinkPreviewAlbumMedia {
    fn from(val: crate::types::LinkPreviewAlbumMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewAlbumMediaVideo`] into [`LinkPreviewAlbumMedia`].
impl From<crate::types::LinkPreviewAlbumMediaVideo> for LinkPreviewAlbumMedia {
    fn from(val: crate::types::LinkPreviewAlbumMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Describes type of link preview
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewType {
    /// The link is a link to a media album consisting of photos and videos
    #[serde(rename(serialize = "linkPreviewTypeAlbum", deserialize = "linkPreviewTypeAlbum"))]
    Album(Box<crate::types::LinkPreviewTypeAlbum>),
    /// The link is a link to an animation
    #[serde(rename(serialize = "linkPreviewTypeAnimation", deserialize = "linkPreviewTypeAnimation"))]
    Animation(Box<crate::types::LinkPreviewTypeAnimation>),
    /// The link is a link to an app at App Store or Google Play
    #[serde(rename(serialize = "linkPreviewTypeApp", deserialize = "linkPreviewTypeApp"))]
    App(Box<crate::types::LinkPreviewTypeApp>),
    /// The link is a link to a website
    #[serde(rename(serialize = "linkPreviewTypeArticle", deserialize = "linkPreviewTypeArticle"))]
    Article(Box<crate::types::LinkPreviewTypeArticle>),
    /// The link is a link to an audio
    #[serde(rename(serialize = "linkPreviewTypeAudio", deserialize = "linkPreviewTypeAudio"))]
    Audio(Box<crate::types::LinkPreviewTypeAudio>),
    /// The link is a link to a background. Link preview title and description are available only for filled backgrounds
    #[serde(rename(serialize = "linkPreviewTypeBackground", deserialize = "linkPreviewTypeBackground"))]
    Background(Box<crate::types::LinkPreviewTypeBackground>),
    /// The link is a link to boost a channel chat
    #[serde(rename(serialize = "linkPreviewTypeChannelBoost", deserialize = "linkPreviewTypeChannelBoost"))]
    ChannelBoost(Box<crate::types::LinkPreviewTypeChannelBoost>),
    /// The link is a link to a chat
    #[serde(rename(serialize = "linkPreviewTypeChat", deserialize = "linkPreviewTypeChat"))]
    Chat(Box<crate::types::LinkPreviewTypeChat>),
    /// The link is a link to a direct messages chat of a channel
    #[serde(rename(serialize = "linkPreviewTypeDirectMessagesChat", deserialize = "linkPreviewTypeDirectMessagesChat"))]
    DirectMessagesChat(Box<crate::types::LinkPreviewTypeDirectMessagesChat>),
    /// The link is a link to a general file
    #[serde(rename(serialize = "linkPreviewTypeDocument", deserialize = "linkPreviewTypeDocument"))]
    Document(Box<crate::types::LinkPreviewTypeDocument>),
    /// The link is a link to an animation player
    #[serde(rename(serialize = "linkPreviewTypeEmbeddedAnimationPlayer", deserialize = "linkPreviewTypeEmbeddedAnimationPlayer"))]
    EmbeddedAnimationPlayer(Box<crate::types::LinkPreviewTypeEmbeddedAnimationPlayer>),
    /// The link is a link to an audio player
    #[serde(rename(serialize = "linkPreviewTypeEmbeddedAudioPlayer", deserialize = "linkPreviewTypeEmbeddedAudioPlayer"))]
    EmbeddedAudioPlayer(Box<crate::types::LinkPreviewTypeEmbeddedAudioPlayer>),
    /// The link is a link to a video player
    #[serde(rename(serialize = "linkPreviewTypeEmbeddedVideoPlayer", deserialize = "linkPreviewTypeEmbeddedVideoPlayer"))]
    EmbeddedVideoPlayer(Box<crate::types::LinkPreviewTypeEmbeddedVideoPlayer>),
    /// The link is a link to an audio file
    #[serde(rename(serialize = "linkPreviewTypeExternalAudio", deserialize = "linkPreviewTypeExternalAudio"))]
    ExternalAudio(Box<crate::types::LinkPreviewTypeExternalAudio>),
    /// The link is a link to a video file
    #[serde(rename(serialize = "linkPreviewTypeExternalVideo", deserialize = "linkPreviewTypeExternalVideo"))]
    ExternalVideo(Box<crate::types::LinkPreviewTypeExternalVideo>),
    /// The link is a link to a gift auction
    #[serde(rename(serialize = "linkPreviewTypeGiftAuction", deserialize = "linkPreviewTypeGiftAuction"))]
    GiftAuction(Box<crate::types::LinkPreviewTypeGiftAuction>),
    /// The link is a link to a gift collection
    #[serde(rename(serialize = "linkPreviewTypeGiftCollection", deserialize = "linkPreviewTypeGiftCollection"))]
    GiftCollection(Box<crate::types::LinkPreviewTypeGiftCollection>),
    /// The link is a link to a group call that isn't bound to a chat
    #[serde(rename(serialize = "linkPreviewTypeGroupCall", deserialize = "linkPreviewTypeGroupCall"))]
    GroupCall,
    /// The link is a link to an invoice
    #[serde(rename(serialize = "linkPreviewTypeInvoice", deserialize = "linkPreviewTypeInvoice"))]
    Invoice,
    /// The link is a link to a live story group call
    #[serde(rename(serialize = "linkPreviewTypeLiveStory", deserialize = "linkPreviewTypeLiveStory"))]
    LiveStory(Box<crate::types::LinkPreviewTypeLiveStory>),
    /// The link is a link to a text or a poll Telegram message
    #[serde(rename(serialize = "linkPreviewTypeMessage", deserialize = "linkPreviewTypeMessage"))]
    Message,
    /// The link is a link to a photo
    #[serde(rename(serialize = "linkPreviewTypePhoto", deserialize = "linkPreviewTypePhoto"))]
    Photo(Box<crate::types::LinkPreviewTypePhoto>),
    /// The link is a link to a Telegram Premium gift code
    #[serde(rename(serialize = "linkPreviewTypePremiumGiftCode", deserialize = "linkPreviewTypePremiumGiftCode"))]
    PremiumGiftCode,
    /// The link is a link to a dialog for creating of a managed bot
    #[serde(rename(serialize = "linkPreviewTypeRequestManagedBot", deserialize = "linkPreviewTypeRequestManagedBot"))]
    RequestManagedBot,
    /// The link is a link to a shareable chat folder
    #[serde(rename(serialize = "linkPreviewTypeShareableChatFolder", deserialize = "linkPreviewTypeShareableChatFolder"))]
    ShareableChatFolder,
    /// The link is a link to a sticker
    #[serde(rename(serialize = "linkPreviewTypeSticker", deserialize = "linkPreviewTypeSticker"))]
    Sticker(Box<crate::types::LinkPreviewTypeSticker>),
    /// The link is a link to a sticker set
    #[serde(rename(serialize = "linkPreviewTypeStickerSet", deserialize = "linkPreviewTypeStickerSet"))]
    StickerSet(Box<crate::types::LinkPreviewTypeStickerSet>),
    /// The link is a link to a story. Link preview description is unavailable
    #[serde(rename(serialize = "linkPreviewTypeStory", deserialize = "linkPreviewTypeStory"))]
    Story(Box<crate::types::LinkPreviewTypeStory>),
    /// The link is a link to an album of stories
    #[serde(rename(serialize = "linkPreviewTypeStoryAlbum", deserialize = "linkPreviewTypeStoryAlbum"))]
    StoryAlbum(Box<crate::types::LinkPreviewTypeStoryAlbum>),
    /// The link is a link to boost a supergroup chat
    #[serde(rename(serialize = "linkPreviewTypeSupergroupBoost", deserialize = "linkPreviewTypeSupergroupBoost"))]
    SupergroupBoost(Box<crate::types::LinkPreviewTypeSupergroupBoost>),
    /// The link is a link to a text composition style
    #[serde(rename(serialize = "linkPreviewTypeTextCompositionStyle", deserialize = "linkPreviewTypeTextCompositionStyle"))]
    TextCompositionStyle(Box<crate::types::LinkPreviewTypeTextCompositionStyle>),
    /// The link is a link to a cloud theme. TDLib has no theme support yet
    #[serde(rename(serialize = "linkPreviewTypeTheme", deserialize = "linkPreviewTypeTheme"))]
    Theme(Box<crate::types::LinkPreviewTypeTheme>),
    /// The link preview type is unsupported yet
    #[serde(rename(serialize = "linkPreviewTypeUnsupported", deserialize = "linkPreviewTypeUnsupported"))]
    Unsupported,
    /// The link is a link to an upgraded gift
    #[serde(rename(serialize = "linkPreviewTypeUpgradedGift", deserialize = "linkPreviewTypeUpgradedGift"))]
    UpgradedGift(Box<crate::types::LinkPreviewTypeUpgradedGift>),
    /// The link is a link to a user
    #[serde(rename(serialize = "linkPreviewTypeUser", deserialize = "linkPreviewTypeUser"))]
    User(Box<crate::types::LinkPreviewTypeUser>),
    /// The link is a link to a video
    #[serde(rename(serialize = "linkPreviewTypeVideo", deserialize = "linkPreviewTypeVideo"))]
    Video(Box<crate::types::LinkPreviewTypeVideo>),
    /// The link is a link to a video chat
    #[serde(rename(serialize = "linkPreviewTypeVideoChat", deserialize = "linkPreviewTypeVideoChat"))]
    VideoChat(Box<crate::types::LinkPreviewTypeVideoChat>),
    /// The link is a link to a video note message
    #[serde(rename(serialize = "linkPreviewTypeVideoNote", deserialize = "linkPreviewTypeVideoNote"))]
    VideoNote(Box<crate::types::LinkPreviewTypeVideoNote>),
    /// The link is a link to a voice note message
    #[serde(rename(serialize = "linkPreviewTypeVoiceNote", deserialize = "linkPreviewTypeVoiceNote"))]
    VoiceNote(Box<crate::types::LinkPreviewTypeVoiceNote>),
    /// The link is a link to a Web App
    #[serde(rename(serialize = "linkPreviewTypeWebApp", deserialize = "linkPreviewTypeWebApp"))]
    WebApp(Box<crate::types::LinkPreviewTypeWebApp>),
}

impl LinkPreviewType {
    /// Convenience constructor to create a [`LinkPreviewType::Album`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn album(val: crate::types::LinkPreviewTypeAlbum) -> Self {
        Self::Album(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::LinkPreviewTypeAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::App`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn app(val: crate::types::LinkPreviewTypeApp) -> Self {
        Self::App(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Article`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn article(val: crate::types::LinkPreviewTypeArticle) -> Self {
        Self::Article(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::LinkPreviewTypeAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Background`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn background(val: crate::types::LinkPreviewTypeBackground) -> Self {
        Self::Background(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::ChannelBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel_boost(val: crate::types::LinkPreviewTypeChannelBoost) -> Self {
        Self::ChannelBoost(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::LinkPreviewTypeChat) -> Self {
        Self::Chat(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::DirectMessagesChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn direct_messages_chat(val: crate::types::LinkPreviewTypeDirectMessagesChat) -> Self {
        Self::DirectMessagesChat(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::LinkPreviewTypeDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::EmbeddedAnimationPlayer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn embedded_animation_player(val: crate::types::LinkPreviewTypeEmbeddedAnimationPlayer) -> Self {
        Self::EmbeddedAnimationPlayer(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::EmbeddedAudioPlayer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn embedded_audio_player(val: crate::types::LinkPreviewTypeEmbeddedAudioPlayer) -> Self {
        Self::EmbeddedAudioPlayer(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::EmbeddedVideoPlayer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn embedded_video_player(val: crate::types::LinkPreviewTypeEmbeddedVideoPlayer) -> Self {
        Self::EmbeddedVideoPlayer(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::ExternalAudio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn external_audio(val: crate::types::LinkPreviewTypeExternalAudio) -> Self {
        Self::ExternalAudio(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::ExternalVideo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn external_video(val: crate::types::LinkPreviewTypeExternalVideo) -> Self {
        Self::ExternalVideo(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::GiftAuction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction(val: crate::types::LinkPreviewTypeGiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::GiftCollection`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_collection(val: crate::types::LinkPreviewTypeGiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::LiveStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_story(val: crate::types::LinkPreviewTypeLiveStory) -> Self {
        Self::LiveStory(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::LinkPreviewTypePhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::LinkPreviewTypeSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::StickerSet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set(val: crate::types::LinkPreviewTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::LinkPreviewTypeStory) -> Self {
        Self::Story(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::StoryAlbum`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_album(val: crate::types::LinkPreviewTypeStoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::SupergroupBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup_boost(val: crate::types::LinkPreviewTypeSupergroupBoost) -> Self {
        Self::SupergroupBoost(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::TextCompositionStyle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_composition_style(val: crate::types::LinkPreviewTypeTextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Theme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn theme(val: crate::types::LinkPreviewTypeTheme) -> Self {
        Self::Theme(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::LinkPreviewTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::LinkPreviewTypeUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::LinkPreviewTypeVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::VideoChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_chat(val: crate::types::LinkPreviewTypeVideoChat) -> Self {
        Self::VideoChat(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::VideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_note(val: crate::types::LinkPreviewTypeVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::LinkPreviewTypeVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`LinkPreviewType::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::LinkPreviewTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::LinkPreviewTypeAlbum`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeAlbum> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeAlbum) -> Self {
        Self::Album(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeAnimation`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeAnimation> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeApp`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeApp> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeApp) -> Self {
        Self::App(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeArticle`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeArticle> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeArticle) -> Self {
        Self::Article(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeAudio`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeAudio> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeBackground`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeBackground> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeBackground) -> Self {
        Self::Background(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeChannelBoost`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeChannelBoost> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeChannelBoost) -> Self {
        Self::ChannelBoost(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeChat`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeChat> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeDirectMessagesChat`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeDirectMessagesChat> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeDirectMessagesChat) -> Self {
        Self::DirectMessagesChat(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeDocument`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeDocument> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeEmbeddedAnimationPlayer`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeEmbeddedAnimationPlayer> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeEmbeddedAnimationPlayer) -> Self {
        Self::EmbeddedAnimationPlayer(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeEmbeddedAudioPlayer`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeEmbeddedAudioPlayer> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeEmbeddedAudioPlayer) -> Self {
        Self::EmbeddedAudioPlayer(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeEmbeddedVideoPlayer`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeEmbeddedVideoPlayer> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeEmbeddedVideoPlayer) -> Self {
        Self::EmbeddedVideoPlayer(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeExternalAudio`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeExternalAudio> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeExternalAudio) -> Self {
        Self::ExternalAudio(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeExternalVideo`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeExternalVideo> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeExternalVideo) -> Self {
        Self::ExternalVideo(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeGiftAuction`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeGiftAuction> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeGiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeGiftCollection`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeGiftCollection> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeGiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeLiveStory`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeLiveStory> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeLiveStory) -> Self {
        Self::LiveStory(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypePhoto`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypePhoto> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypePhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeSticker`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeSticker> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeStickerSet`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeStickerSet> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeStory`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeStory> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeStoryAlbum`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeStoryAlbum> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeStoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeSupergroupBoost`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeSupergroupBoost> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeSupergroupBoost) -> Self {
        Self::SupergroupBoost(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeTextCompositionStyle`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeTextCompositionStyle> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeTextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeTheme`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeTheme> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeTheme) -> Self {
        Self::Theme(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeUpgradedGift`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeUpgradedGift> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeUser`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeUser> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeVideo`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeVideo> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeVideoChat`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeVideoChat> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeVideoChat) -> Self {
        Self::VideoChat(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeVideoNote`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeVideoNote> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeVoiceNote`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeVoiceNote> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::LinkPreviewTypeWebApp`] into [`LinkPreviewType`].
impl From<crate::types::LinkPreviewTypeWebApp> for LinkPreviewType {
    fn from(val: crate::types::LinkPreviewTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// TDLib `LinkPreview` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreview {
    /// Describes a link preview
    #[serde(rename(serialize = "linkPreview", deserialize = "linkPreview"))]
    LinkPreview(Box<crate::types::LinkPreview>),
}

impl LinkPreview {
    /// Convenience constructor to create a [`LinkPreview::LinkPreview`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link_preview(val: crate::types::LinkPreview) -> Self {
        Self::LinkPreview(Box::new(val))
    }

}

/// Converts a [`crate::types::LinkPreview`] into [`LinkPreview`].
impl From<crate::types::LinkPreview> for LinkPreview {
    fn from(val: crate::types::LinkPreview) -> Self {
        Self::LinkPreview(Box::new(val))
    }
}

/// TDLib `CountryInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CountryInfo {
    /// Contains information about a country
    #[serde(rename(serialize = "countryInfo", deserialize = "countryInfo"))]
    CountryInfo(Box<crate::types::CountryInfo>),
}

impl CountryInfo {
    /// Convenience constructor to create a [`CountryInfo::CountryInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn country_info(val: crate::types::CountryInfo) -> Self {
        Self::CountryInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::CountryInfo`] into [`CountryInfo`].
impl From<crate::types::CountryInfo> for CountryInfo {
    fn from(val: crate::types::CountryInfo) -> Self {
        Self::CountryInfo(Box::new(val))
    }
}

/// TDLib `Countries` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Countries {
    /// Contains information about countries
    #[serde(rename(serialize = "countries", deserialize = "countries"))]
    Countries(Box<crate::types::Countries>),
}

impl Countries {
    /// Convenience constructor to create a [`Countries::Countries`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn countries(val: crate::types::Countries) -> Self {
        Self::Countries(Box::new(val))
    }

}

/// Converts a [`crate::types::Countries`] into [`Countries`].
impl From<crate::types::Countries> for Countries {
    fn from(val: crate::types::Countries) -> Self {
        Self::Countries(Box::new(val))
    }
}

/// TDLib `PhoneNumberInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberInfo {
    /// Contains information about a phone number
    #[serde(rename(serialize = "phoneNumberInfo", deserialize = "phoneNumberInfo"))]
    PhoneNumberInfo(Box<crate::types::PhoneNumberInfo>),
}

impl PhoneNumberInfo {
    /// Convenience constructor to create a [`PhoneNumberInfo::PhoneNumberInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number_info(val: crate::types::PhoneNumberInfo) -> Self {
        Self::PhoneNumberInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::PhoneNumberInfo`] into [`PhoneNumberInfo`].
impl From<crate::types::PhoneNumberInfo> for PhoneNumberInfo {
    fn from(val: crate::types::PhoneNumberInfo) -> Self {
        Self::PhoneNumberInfo(Box::new(val))
    }
}

/// Describes a collectible item that can be purchased at https:fragment.com
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CollectibleItemType {
    /// A username
    #[serde(rename(serialize = "collectibleItemTypeUsername", deserialize = "collectibleItemTypeUsername"))]
    Username(Box<crate::types::CollectibleItemTypeUsername>),
    /// A phone number
    #[serde(rename(serialize = "collectibleItemTypePhoneNumber", deserialize = "collectibleItemTypePhoneNumber"))]
    PhoneNumber(Box<crate::types::CollectibleItemTypePhoneNumber>),
}

impl CollectibleItemType {
    /// Convenience constructor to create a [`CollectibleItemType::Username`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn username(val: crate::types::CollectibleItemTypeUsername) -> Self {
        Self::Username(Box::new(val))
    }

    /// Convenience constructor to create a [`CollectibleItemType::PhoneNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number(val: crate::types::CollectibleItemTypePhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }

}

/// Converts a [`crate::types::CollectibleItemTypeUsername`] into [`CollectibleItemType`].
impl From<crate::types::CollectibleItemTypeUsername> for CollectibleItemType {
    fn from(val: crate::types::CollectibleItemTypeUsername) -> Self {
        Self::Username(Box::new(val))
    }
}

/// Converts a [`crate::types::CollectibleItemTypePhoneNumber`] into [`CollectibleItemType`].
impl From<crate::types::CollectibleItemTypePhoneNumber> for CollectibleItemType {
    fn from(val: crate::types::CollectibleItemTypePhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }
}

/// TDLib `CollectibleItemInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CollectibleItemInfo {
    /// Contains information about a collectible item and its last purchase
    #[serde(rename(serialize = "collectibleItemInfo", deserialize = "collectibleItemInfo"))]
    CollectibleItemInfo(Box<crate::types::CollectibleItemInfo>),
}

impl CollectibleItemInfo {
    /// Convenience constructor to create a [`CollectibleItemInfo::CollectibleItemInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn collectible_item_info(val: crate::types::CollectibleItemInfo) -> Self {
        Self::CollectibleItemInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::CollectibleItemInfo`] into [`CollectibleItemInfo`].
impl From<crate::types::CollectibleItemInfo> for CollectibleItemInfo {
    fn from(val: crate::types::CollectibleItemInfo) -> Self {
        Self::CollectibleItemInfo(Box::new(val))
    }
}

/// TDLib `BankCardActionOpenUrl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BankCardActionOpenUrl {
    /// Describes an action associated with a bank card number
    #[serde(rename(serialize = "bankCardActionOpenUrl", deserialize = "bankCardActionOpenUrl"))]
    BankCardActionOpenUrl(Box<crate::types::BankCardActionOpenUrl>),
}

impl BankCardActionOpenUrl {
    /// Convenience constructor to create a [`BankCardActionOpenUrl::BankCardActionOpenUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bank_card_action_open_url(val: crate::types::BankCardActionOpenUrl) -> Self {
        Self::BankCardActionOpenUrl(Box::new(val))
    }

}

/// Converts a [`crate::types::BankCardActionOpenUrl`] into [`BankCardActionOpenUrl`].
impl From<crate::types::BankCardActionOpenUrl> for BankCardActionOpenUrl {
    fn from(val: crate::types::BankCardActionOpenUrl) -> Self {
        Self::BankCardActionOpenUrl(Box::new(val))
    }
}

/// TDLib `BankCardInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BankCardInfo {
    /// Information about a bank card
    #[serde(rename(serialize = "bankCardInfo", deserialize = "bankCardInfo"))]
    BankCardInfo(Box<crate::types::BankCardInfo>),
}

impl BankCardInfo {
    /// Convenience constructor to create a [`BankCardInfo::BankCardInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bank_card_info(val: crate::types::BankCardInfo) -> Self {
        Self::BankCardInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::BankCardInfo`] into [`BankCardInfo`].
impl From<crate::types::BankCardInfo> for BankCardInfo {
    fn from(val: crate::types::BankCardInfo) -> Self {
        Self::BankCardInfo(Box::new(val))
    }
}

/// TDLib `Address` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Address {
    /// Describes an address
    #[serde(rename(serialize = "address", deserialize = "address"))]
    Address(Box<crate::types::Address>),
}

impl Address {
    /// Convenience constructor to create a [`Address::Address`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn address(val: crate::types::Address) -> Self {
        Self::Address(Box::new(val))
    }

}

/// Converts a [`crate::types::Address`] into [`Address`].
impl From<crate::types::Address> for Address {
    fn from(val: crate::types::Address) -> Self {
        Self::Address(Box::new(val))
    }
}

/// TDLib `LocationAddress` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LocationAddress {
    /// Describes an address of a location
    #[serde(rename(serialize = "locationAddress", deserialize = "locationAddress"))]
    LocationAddress(Box<crate::types::LocationAddress>),
}

impl LocationAddress {
    /// Convenience constructor to create a [`LocationAddress::LocationAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location_address(val: crate::types::LocationAddress) -> Self {
        Self::LocationAddress(Box::new(val))
    }

}

/// Converts a [`crate::types::LocationAddress`] into [`LocationAddress`].
impl From<crate::types::LocationAddress> for LocationAddress {
    fn from(val: crate::types::LocationAddress) -> Self {
        Self::LocationAddress(Box::new(val))
    }
}

/// TDLib `LabeledPricePart` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LabeledPricePart {
    /// Portion of the price of a product (e.g., "delivery cost", "tax amount")
    #[serde(rename(serialize = "labeledPricePart", deserialize = "labeledPricePart"))]
    LabeledPricePart(Box<crate::types::LabeledPricePart>),
}

impl LabeledPricePart {
    /// Convenience constructor to create a [`LabeledPricePart::LabeledPricePart`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn labeled_price_part(val: crate::types::LabeledPricePart) -> Self {
        Self::LabeledPricePart(Box::new(val))
    }

}

/// Converts a [`crate::types::LabeledPricePart`] into [`LabeledPricePart`].
impl From<crate::types::LabeledPricePart> for LabeledPricePart {
    fn from(val: crate::types::LabeledPricePart) -> Self {
        Self::LabeledPricePart(Box::new(val))
    }
}

/// TDLib `OrderInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OrderInfo {
    /// Order information
    #[serde(rename(serialize = "orderInfo", deserialize = "orderInfo"))]
    OrderInfo(Box<crate::types::OrderInfo>),
}

impl OrderInfo {
    /// Convenience constructor to create a [`OrderInfo::OrderInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn order_info(val: crate::types::OrderInfo) -> Self {
        Self::OrderInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::OrderInfo`] into [`OrderInfo`].
impl From<crate::types::OrderInfo> for OrderInfo {
    fn from(val: crate::types::OrderInfo) -> Self {
        Self::OrderInfo(Box::new(val))
    }
}

/// TDLib `SavedCredentials` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedCredentials {
    /// Contains information about saved payment credentials
    #[serde(rename(serialize = "savedCredentials", deserialize = "savedCredentials"))]
    SavedCredentials(Box<crate::types::SavedCredentials>),
}

impl SavedCredentials {
    /// Convenience constructor to create a [`SavedCredentials::SavedCredentials`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_credentials(val: crate::types::SavedCredentials) -> Self {
        Self::SavedCredentials(Box::new(val))
    }

}

/// Converts a [`crate::types::SavedCredentials`] into [`SavedCredentials`].
impl From<crate::types::SavedCredentials> for SavedCredentials {
    fn from(val: crate::types::SavedCredentials) -> Self {
        Self::SavedCredentials(Box::new(val))
    }
}

/// Contains information about the payment method chosen by the user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputCredentials {
    /// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
    #[serde(rename(serialize = "inputCredentialsSaved", deserialize = "inputCredentialsSaved"))]
    Saved(Box<crate::types::InputCredentialsSaved>),
    /// Applies if a user enters new credentials on a payment provider website
    #[serde(rename(serialize = "inputCredentialsNew", deserialize = "inputCredentialsNew"))]
    New(Box<crate::types::InputCredentialsNew>),
    /// Applies if a user enters new credentials using Apple Pay
    #[serde(rename(serialize = "inputCredentialsApplePay", deserialize = "inputCredentialsApplePay"))]
    ApplePay(Box<crate::types::InputCredentialsApplePay>),
    /// Applies if a user enters new credentials using Google Pay
    #[serde(rename(serialize = "inputCredentialsGooglePay", deserialize = "inputCredentialsGooglePay"))]
    GooglePay(Box<crate::types::InputCredentialsGooglePay>),
}

impl InputCredentials {
    /// Convenience constructor to create a [`InputCredentials::Saved`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved(val: crate::types::InputCredentialsSaved) -> Self {
        Self::Saved(Box::new(val))
    }

    /// Convenience constructor to create a [`InputCredentials::New`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new(val: crate::types::InputCredentialsNew) -> Self {
        Self::New(Box::new(val))
    }

    /// Convenience constructor to create a [`InputCredentials::ApplePay`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn apple_pay(val: crate::types::InputCredentialsApplePay) -> Self {
        Self::ApplePay(Box::new(val))
    }

    /// Convenience constructor to create a [`InputCredentials::GooglePay`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn google_pay(val: crate::types::InputCredentialsGooglePay) -> Self {
        Self::GooglePay(Box::new(val))
    }

}

/// Converts a [`crate::types::InputCredentialsSaved`] into [`InputCredentials`].
impl From<crate::types::InputCredentialsSaved> for InputCredentials {
    fn from(val: crate::types::InputCredentialsSaved) -> Self {
        Self::Saved(Box::new(val))
    }
}

/// Converts a [`crate::types::InputCredentialsNew`] into [`InputCredentials`].
impl From<crate::types::InputCredentialsNew> for InputCredentials {
    fn from(val: crate::types::InputCredentialsNew) -> Self {
        Self::New(Box::new(val))
    }
}

/// Converts a [`crate::types::InputCredentialsApplePay`] into [`InputCredentials`].
impl From<crate::types::InputCredentialsApplePay> for InputCredentials {
    fn from(val: crate::types::InputCredentialsApplePay) -> Self {
        Self::ApplePay(Box::new(val))
    }
}

/// Converts a [`crate::types::InputCredentialsGooglePay`] into [`InputCredentials`].
impl From<crate::types::InputCredentialsGooglePay> for InputCredentials {
    fn from(val: crate::types::InputCredentialsGooglePay) -> Self {
        Self::GooglePay(Box::new(val))
    }
}

/// TDLib `ValidatedOrderInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ValidatedOrderInfo {
    /// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
    #[serde(rename(serialize = "validatedOrderInfo", deserialize = "validatedOrderInfo"))]
    ValidatedOrderInfo(Box<crate::types::ValidatedOrderInfo>),
}

impl ValidatedOrderInfo {
    /// Convenience constructor to create a [`ValidatedOrderInfo::ValidatedOrderInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn validated_order_info(val: crate::types::ValidatedOrderInfo) -> Self {
        Self::ValidatedOrderInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ValidatedOrderInfo`] into [`ValidatedOrderInfo`].
impl From<crate::types::ValidatedOrderInfo> for ValidatedOrderInfo {
    fn from(val: crate::types::ValidatedOrderInfo) -> Self {
        Self::ValidatedOrderInfo(Box::new(val))
    }
}

/// Describes a paid media
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidMedia {
    /// The media is hidden until the invoice is paid
    #[serde(rename(serialize = "paidMediaPreview", deserialize = "paidMediaPreview"))]
    Preview(Box<crate::types::PaidMediaPreview>),
    /// The media is a photo
    #[serde(rename(serialize = "paidMediaPhoto", deserialize = "paidMediaPhoto"))]
    Photo(Box<crate::types::PaidMediaPhoto>),
    /// The media is a video
    #[serde(rename(serialize = "paidMediaVideo", deserialize = "paidMediaVideo"))]
    Video(Box<crate::types::PaidMediaVideo>),
    /// The media is unsupported
    #[serde(rename(serialize = "paidMediaUnsupported", deserialize = "paidMediaUnsupported"))]
    Unsupported,
}

impl PaidMedia {
    /// Convenience constructor to create a [`PaidMedia::Preview`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn preview(val: crate::types::PaidMediaPreview) -> Self {
        Self::Preview(Box::new(val))
    }

    /// Convenience constructor to create a [`PaidMedia::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::PaidMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`PaidMedia::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::PaidMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::PaidMediaPreview`] into [`PaidMedia`].
impl From<crate::types::PaidMediaPreview> for PaidMedia {
    fn from(val: crate::types::PaidMediaPreview) -> Self {
        Self::Preview(Box::new(val))
    }
}

/// Converts a [`crate::types::PaidMediaPhoto`] into [`PaidMedia`].
impl From<crate::types::PaidMediaPhoto> for PaidMedia {
    fn from(val: crate::types::PaidMediaPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::PaidMediaVideo`] into [`PaidMedia`].
impl From<crate::types::PaidMediaVideo> for PaidMedia {
    fn from(val: crate::types::PaidMediaVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// TDLib `Date` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Date {
    /// Represents a date according to the Gregorian calendar
    #[serde(rename(serialize = "date", deserialize = "date"))]
    Date(Box<crate::types::Date>),
}

impl Date {
    /// Convenience constructor to create a [`Date::Date`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn date(val: crate::types::Date) -> Self {
        Self::Date(Box::new(val))
    }

}

/// Converts a [`crate::types::Date`] into [`Date`].
impl From<crate::types::Date> for Date {
    fn from(val: crate::types::Date) -> Self {
        Self::Date(Box::new(val))
    }
}

/// TDLib `PersonalDetails` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PersonalDetails {
    /// Contains the user's personal details
    #[serde(rename(serialize = "personalDetails", deserialize = "personalDetails"))]
    PersonalDetails(Box<crate::types::PersonalDetails>),
}

impl PersonalDetails {
    /// Convenience constructor to create a [`PersonalDetails::PersonalDetails`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn personal_details(val: crate::types::PersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }

}

/// Converts a [`crate::types::PersonalDetails`] into [`PersonalDetails`].
impl From<crate::types::PersonalDetails> for PersonalDetails {
    fn from(val: crate::types::PersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }
}

/// TDLib `EncryptedCredentials` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EncryptedCredentials {
    /// Contains encrypted Telegram Passport data credentials
    #[serde(rename(serialize = "encryptedCredentials", deserialize = "encryptedCredentials"))]
    EncryptedCredentials(Box<crate::types::EncryptedCredentials>),
}

impl EncryptedCredentials {
    /// Convenience constructor to create a [`EncryptedCredentials::EncryptedCredentials`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn encrypted_credentials(val: crate::types::EncryptedCredentials) -> Self {
        Self::EncryptedCredentials(Box::new(val))
    }

}

/// Converts a [`crate::types::EncryptedCredentials`] into [`EncryptedCredentials`].
impl From<crate::types::EncryptedCredentials> for EncryptedCredentials {
    fn from(val: crate::types::EncryptedCredentials) -> Self {
        Self::EncryptedCredentials(Box::new(val))
    }
}

/// Describes precision with which to show a date or a time
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateTimePartPrecision {
    /// Don't show the date or time
    #[serde(rename(serialize = "dateTimePartPrecisionNone", deserialize = "dateTimePartPrecisionNone"))]
    None,
    /// Show the date or time in a short way (17.03.22 or 22:45)
    #[serde(rename(serialize = "dateTimePartPrecisionShort", deserialize = "dateTimePartPrecisionShort"))]
    Short,
    /// Show the date or time in a long way (March 17, 2022 or 22:45:00)
    #[serde(rename(serialize = "dateTimePartPrecisionLong", deserialize = "dateTimePartPrecisionLong"))]
    Long,
}

/// Describes date and time formatting
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateTimeFormattingType {
    /// The time must be shown relative to the current time ([in ] X seconds, minutes, hours, days, months, years [ago])
    #[serde(rename(serialize = "dateTimeFormattingTypeRelative", deserialize = "dateTimeFormattingTypeRelative"))]
    Relative,
    /// The date and time must be shown as absolute timestamps
    #[serde(rename(serialize = "dateTimeFormattingTypeAbsolute", deserialize = "dateTimeFormattingTypeAbsolute"))]
    Absolute(Box<crate::types::DateTimeFormattingTypeAbsolute>),
}

impl DateTimeFormattingType {
    /// Convenience constructor to create a [`DateTimeFormattingType::Absolute`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn absolute(val: crate::types::DateTimeFormattingTypeAbsolute) -> Self {
        Self::Absolute(Box::new(val))
    }

}

/// Converts a [`crate::types::DateTimeFormattingTypeAbsolute`] into [`DateTimeFormattingType`].
impl From<crate::types::DateTimeFormattingTypeAbsolute> for DateTimeFormattingType {
    fn from(val: crate::types::DateTimeFormattingTypeAbsolute) -> Self {
        Self::Absolute(Box::new(val))
    }
}

/// Represents a change of a text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiffEntityType {
    /// Addition of some text
    #[serde(rename(serialize = "diffEntityTypeInsert", deserialize = "diffEntityTypeInsert"))]
    Insert,
    /// Change of some text
    #[serde(rename(serialize = "diffEntityTypeReplace", deserialize = "diffEntityTypeReplace"))]
    Replace(Box<crate::types::DiffEntityTypeReplace>),
    /// Removal of some text
    #[serde(rename(serialize = "diffEntityTypeDelete", deserialize = "diffEntityTypeDelete"))]
    Delete,
}

impl DiffEntityType {
    /// Convenience constructor to create a [`DiffEntityType::Replace`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn replace(val: crate::types::DiffEntityTypeReplace) -> Self {
        Self::Replace(Box::new(val))
    }

}

/// Converts a [`crate::types::DiffEntityTypeReplace`] into [`DiffEntityType`].
impl From<crate::types::DiffEntityTypeReplace> for DiffEntityType {
    fn from(val: crate::types::DiffEntityTypeReplace) -> Self {
        Self::Replace(Box::new(val))
    }
}

/// TDLib `InputVoiceNote` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputVoiceNote {
    /// A video note to be sent
    #[serde(rename(serialize = "inputVoiceNote", deserialize = "inputVoiceNote"))]
    InputVoiceNote(Box<crate::types::InputVoiceNote>),
}

impl InputVoiceNote {
    /// Convenience constructor to create a [`InputVoiceNote::InputVoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_voice_note(val: crate::types::InputVoiceNote) -> Self {
        Self::InputVoiceNote(Box::new(val))
    }

}

/// Converts a [`crate::types::InputVoiceNote`] into [`InputVoiceNote`].
impl From<crate::types::InputVoiceNote> for InputVoiceNote {
    fn from(val: crate::types::InputVoiceNote) -> Self {
        Self::InputVoiceNote(Box::new(val))
    }
}

/// Describes type of paid media to send
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPaidMediaType {
    /// The media is a photo. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
    #[serde(rename(serialize = "inputPaidMediaTypePhoto", deserialize = "inputPaidMediaTypePhoto"))]
    Photo(Box<crate::types::InputPaidMediaTypePhoto>),
    /// The media is a video
    #[serde(rename(serialize = "inputPaidMediaTypeVideo", deserialize = "inputPaidMediaTypeVideo"))]
    Video(Box<crate::types::InputPaidMediaTypeVideo>),
}

impl InputPaidMediaType {
    /// Convenience constructor to create a [`InputPaidMediaType::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InputPaidMediaTypePhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPaidMediaType::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InputPaidMediaTypeVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPaidMediaTypePhoto`] into [`InputPaidMediaType`].
impl From<crate::types::InputPaidMediaTypePhoto> for InputPaidMediaType {
    fn from(val: crate::types::InputPaidMediaTypePhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPaidMediaTypeVideo`] into [`InputPaidMediaType`].
impl From<crate::types::InputPaidMediaTypeVideo> for InputPaidMediaType {
    fn from(val: crate::types::InputPaidMediaTypeVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// TDLib `InputPaidMedia` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPaidMedia {
    /// Describes a paid media to be sent
    #[serde(rename(serialize = "inputPaidMedia", deserialize = "inputPaidMedia"))]
    InputPaidMedia(Box<crate::types::InputPaidMedia>),
}

impl InputPaidMedia {
    /// Convenience constructor to create a [`InputPaidMedia::InputPaidMedia`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_paid_media(val: crate::types::InputPaidMedia) -> Self {
        Self::InputPaidMedia(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPaidMedia`] into [`InputPaidMedia`].
impl From<crate::types::InputPaidMedia> for InputPaidMedia {
    fn from(val: crate::types::InputPaidMedia) -> Self {
        Self::InputPaidMedia(Box::new(val))
    }
}

/// Describes a block of a rich message to send
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPageBlock {
    /// A section heading
    #[serde(rename(serialize = "inputPageBlockSectionHeading", deserialize = "inputPageBlockSectionHeading"))]
    SectionHeading(Box<crate::types::InputPageBlockSectionHeading>),
    /// A text paragraph
    #[serde(rename(serialize = "inputPageBlockParagraph", deserialize = "inputPageBlockParagraph"))]
    Paragraph(Box<crate::types::InputPageBlockParagraph>),
    /// A preformatted text paragraph
    #[serde(rename(serialize = "inputPageBlockPreformatted", deserialize = "inputPageBlockPreformatted"))]
    Preformatted(Box<crate::types::InputPageBlockPreformatted>),
    /// The footer of the page
    #[serde(rename(serialize = "inputPageBlockFooter", deserialize = "inputPageBlockFooter"))]
    Footer(Box<crate::types::InputPageBlockFooter>),
    /// A "Thinking..." placeholder; for pending rich messages only; for bots only
    #[serde(rename(serialize = "inputPageBlockThinking", deserialize = "inputPageBlockThinking"))]
    Thinking(Box<crate::types::InputPageBlockThinking>),
    /// An empty block separating the page
    #[serde(rename(serialize = "inputPageBlockDivider", deserialize = "inputPageBlockDivider"))]
    Divider,
    /// A mathematical expression
    #[serde(rename(serialize = "inputPageBlockMathematicalExpression", deserialize = "inputPageBlockMathematicalExpression"))]
    MathematicalExpression(Box<crate::types::InputPageBlockMathematicalExpression>),
    /// An invisible anchor
    #[serde(rename(serialize = "inputPageBlockAnchor", deserialize = "inputPageBlockAnchor"))]
    Anchor(Box<crate::types::InputPageBlockAnchor>),
    /// A list of data blocks
    #[serde(rename(serialize = "inputPageBlockList", deserialize = "inputPageBlockList"))]
    List(Box<crate::types::InputPageBlockList>),
    /// A block quote
    #[serde(rename(serialize = "inputPageBlockBlockQuote", deserialize = "inputPageBlockBlockQuote"))]
    BlockQuote(Box<crate::types::InputPageBlockBlockQuote>),
    /// An expandable block quote
    #[serde(rename(serialize = "inputPageBlockExpandableBlockQuote", deserialize = "inputPageBlockExpandableBlockQuote"))]
    ExpandableBlockQuote(Box<crate::types::InputPageBlockExpandableBlockQuote>),
    /// A pull quote
    #[serde(rename(serialize = "inputPageBlockPullQuote", deserialize = "inputPageBlockPullQuote"))]
    PullQuote(Box<crate::types::InputPageBlockPullQuote>),
    /// An animation
    #[serde(rename(serialize = "inputPageBlockAnimation", deserialize = "inputPageBlockAnimation"))]
    Animation(Box<crate::types::InputPageBlockAnimation>),
    /// An audio file
    #[serde(rename(serialize = "inputPageBlockAudio", deserialize = "inputPageBlockAudio"))]
    Audio(Box<crate::types::InputPageBlockAudio>),
    /// A general file
    #[serde(rename(serialize = "inputPageBlockDocument", deserialize = "inputPageBlockDocument"))]
    Document(Box<crate::types::InputPageBlockDocument>),
    /// A photo
    #[serde(rename(serialize = "inputPageBlockPhoto", deserialize = "inputPageBlockPhoto"))]
    Photo(Box<crate::types::InputPageBlockPhoto>),
    /// A video
    #[serde(rename(serialize = "inputPageBlockVideo", deserialize = "inputPageBlockVideo"))]
    Video(Box<crate::types::InputPageBlockVideo>),
    /// A voice note
    #[serde(rename(serialize = "inputPageBlockVoiceNote", deserialize = "inputPageBlockVoiceNote"))]
    VoiceNote(Box<crate::types::InputPageBlockVoiceNote>),
    /// A collage
    #[serde(rename(serialize = "inputPageBlockCollage", deserialize = "inputPageBlockCollage"))]
    Collage(Box<crate::types::InputPageBlockCollage>),
    /// A slideshow
    #[serde(rename(serialize = "inputPageBlockSlideshow", deserialize = "inputPageBlockSlideshow"))]
    Slideshow(Box<crate::types::InputPageBlockSlideshow>),
    /// A table
    #[serde(rename(serialize = "inputPageBlockTable", deserialize = "inputPageBlockTable"))]
    Table(Box<crate::types::InputPageBlockTable>),
    /// A collapsible block
    #[serde(rename(serialize = "inputPageBlockDetails", deserialize = "inputPageBlockDetails"))]
    Details(Box<crate::types::InputPageBlockDetails>),
    /// A map. The map's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
    #[serde(rename(serialize = "inputPageBlockMap", deserialize = "inputPageBlockMap"))]
    Map(Box<crate::types::InputPageBlockMap>),
    /// A list of buttons shown in a row
    #[serde(rename(serialize = "inputPageBlockButtonRow", deserialize = "inputPageBlockButtonRow"))]
    ButtonRow(Box<crate::types::InputPageBlockButtonRow>),
}

impl InputPageBlock {
    /// Convenience constructor to create a [`InputPageBlock::SectionHeading`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn section_heading(val: crate::types::InputPageBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Paragraph`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paragraph(val: crate::types::InputPageBlockParagraph) -> Self {
        Self::Paragraph(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Preformatted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn preformatted(val: crate::types::InputPageBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Footer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn footer(val: crate::types::InputPageBlockFooter) -> Self {
        Self::Footer(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Thinking`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn thinking(val: crate::types::InputPageBlockThinking) -> Self {
        Self::Thinking(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::MathematicalExpression`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mathematical_expression(val: crate::types::InputPageBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Anchor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn anchor(val: crate::types::InputPageBlockAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::List`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn list(val: crate::types::InputPageBlockList) -> Self {
        Self::List(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::BlockQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn block_quote(val: crate::types::InputPageBlockBlockQuote) -> Self {
        Self::BlockQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::ExpandableBlockQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn expandable_block_quote(val: crate::types::InputPageBlockExpandableBlockQuote) -> Self {
        Self::ExpandableBlockQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::PullQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pull_quote(val: crate::types::InputPageBlockPullQuote) -> Self {
        Self::PullQuote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::InputPageBlockAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::InputPageBlockAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::InputPageBlockDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InputPageBlockPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InputPageBlockVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::InputPageBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Collage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn collage(val: crate::types::InputPageBlockCollage) -> Self {
        Self::Collage(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Slideshow`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn slideshow(val: crate::types::InputPageBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Table`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn table(val: crate::types::InputPageBlockTable) -> Self {
        Self::Table(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Details`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn details(val: crate::types::InputPageBlockDetails) -> Self {
        Self::Details(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::Map`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn map(val: crate::types::InputPageBlockMap) -> Self {
        Self::Map(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPageBlock::ButtonRow`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn button_row(val: crate::types::InputPageBlockButtonRow) -> Self {
        Self::ButtonRow(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPageBlockSectionHeading`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockSectionHeading> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockSectionHeading) -> Self {
        Self::SectionHeading(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockParagraph`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockParagraph> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockParagraph) -> Self {
        Self::Paragraph(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockPreformatted`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockPreformatted> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockPreformatted) -> Self {
        Self::Preformatted(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockFooter`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockFooter> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockFooter) -> Self {
        Self::Footer(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockThinking`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockThinking> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockThinking) -> Self {
        Self::Thinking(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockMathematicalExpression`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockMathematicalExpression> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockAnchor`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockAnchor> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockList`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockList> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockList) -> Self {
        Self::List(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockBlockQuote`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockBlockQuote> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockBlockQuote) -> Self {
        Self::BlockQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockExpandableBlockQuote`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockExpandableBlockQuote> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockExpandableBlockQuote) -> Self {
        Self::ExpandableBlockQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockPullQuote`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockPullQuote> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockPullQuote) -> Self {
        Self::PullQuote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockAnimation`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockAnimation> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockAudio`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockAudio> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockDocument`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockDocument> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockPhoto`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockPhoto> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockVideo`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockVideo> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockVoiceNote`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockVoiceNote> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockCollage`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockCollage> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockCollage) -> Self {
        Self::Collage(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockSlideshow`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockSlideshow> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockSlideshow) -> Self {
        Self::Slideshow(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockTable`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockTable> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockTable) -> Self {
        Self::Table(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockDetails`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockDetails> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockDetails) -> Self {
        Self::Details(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockMap`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockMap> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockMap) -> Self {
        Self::Map(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPageBlockButtonRow`] into [`InputPageBlock`].
impl From<crate::types::InputPageBlockButtonRow> for InputPageBlock {
    fn from(val: crate::types::InputPageBlockButtonRow) -> Self {
        Self::ButtonRow(Box::new(val))
    }
}

/// TDLib `CurrentWeather` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CurrentWeather {
    /// Describes the current weather
    #[serde(rename(serialize = "currentWeather", deserialize = "currentWeather"))]
    CurrentWeather(Box<crate::types::CurrentWeather>),
}

impl CurrentWeather {
    /// Convenience constructor to create a [`CurrentWeather::CurrentWeather`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn current_weather(val: crate::types::CurrentWeather) -> Self {
        Self::CurrentWeather(Box::new(val))
    }

}

/// Converts a [`crate::types::CurrentWeather`] into [`CurrentWeather`].
impl From<crate::types::CurrentWeather> for CurrentWeather {
    fn from(val: crate::types::CurrentWeather) -> Self {
        Self::CurrentWeather(Box::new(val))
    }
}

/// TDLib `QuickReplyShortcut` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyShortcut {
    /// Describes a shortcut that can be used for a quick reply
    #[serde(rename(serialize = "quickReplyShortcut", deserialize = "quickReplyShortcut"))]
    QuickReplyShortcut(Box<crate::types::QuickReplyShortcut>),
}

impl QuickReplyShortcut {
    /// Convenience constructor to create a [`QuickReplyShortcut::QuickReplyShortcut`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_shortcut(val: crate::types::QuickReplyShortcut) -> Self {
        Self::QuickReplyShortcut(Box::new(val))
    }

}

/// Converts a [`crate::types::QuickReplyShortcut`] into [`QuickReplyShortcut`].
impl From<crate::types::QuickReplyShortcut> for QuickReplyShortcut {
    fn from(val: crate::types::QuickReplyShortcut) -> Self {
        Self::QuickReplyShortcut(Box::new(val))
    }
}

/// Describes a public forward or repost of a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicForward {
    /// Contains a public forward as a message
    #[serde(rename(serialize = "publicForwardMessage", deserialize = "publicForwardMessage"))]
    Message(Box<crate::types::PublicForwardMessage>),
    /// Contains a public repost to a story
    #[serde(rename(serialize = "publicForwardStory", deserialize = "publicForwardStory"))]
    Story(Box<crate::types::PublicForwardStory>),
}

impl PublicForward {
    /// Convenience constructor to create a [`PublicForward::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::PublicForwardMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`PublicForward::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::PublicForwardStory) -> Self {
        Self::Story(Box::new(val))
    }

}

/// Converts a [`crate::types::PublicForwardMessage`] into [`PublicForward`].
impl From<crate::types::PublicForwardMessage> for PublicForward {
    fn from(val: crate::types::PublicForwardMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::PublicForwardStory`] into [`PublicForward`].
impl From<crate::types::PublicForwardStory> for PublicForward {
    fn from(val: crate::types::PublicForwardStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// TDLib `PublicForwards` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicForwards {
    /// Represents a list of public forwards and reposts as a story of a message or a story
    #[serde(rename(serialize = "publicForwards", deserialize = "publicForwards"))]
    PublicForwards(Box<crate::types::PublicForwards>),
}

impl PublicForwards {
    /// Convenience constructor to create a [`PublicForwards::PublicForwards`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn public_forwards(val: crate::types::PublicForwards) -> Self {
        Self::PublicForwards(Box::new(val))
    }

}

/// Converts a [`crate::types::PublicForwards`] into [`PublicForwards`].
impl From<crate::types::PublicForwards> for PublicForwards {
    fn from(val: crate::types::PublicForwards) -> Self {
        Self::PublicForwards(Box::new(val))
    }
}

/// Describes the reason why a code needs to be re-sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ResendCodeReason {
    /// The user requested to resend the code
    #[serde(rename(serialize = "resendCodeReasonUserRequest", deserialize = "resendCodeReasonUserRequest"))]
    UserRequest,
    /// The code is re-sent, because device verification has failed
    #[serde(rename(serialize = "resendCodeReasonVerificationFailed", deserialize = "resendCodeReasonVerificationFailed"))]
    VerificationFailed(Box<crate::types::ResendCodeReasonVerificationFailed>),
}

impl ResendCodeReason {
    /// Convenience constructor to create a [`ResendCodeReason::VerificationFailed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn verification_failed(val: crate::types::ResendCodeReasonVerificationFailed) -> Self {
        Self::VerificationFailed(Box::new(val))
    }

}

/// Converts a [`crate::types::ResendCodeReasonVerificationFailed`] into [`ResendCodeReason`].
impl From<crate::types::ResendCodeReasonVerificationFailed> for ResendCodeReason {
    fn from(val: crate::types::ResendCodeReasonVerificationFailed) -> Self {
        Self::VerificationFailed(Box::new(val))
    }
}

/// TDLib `RtmpUrl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RtmpUrl {
    /// Represents an RTMP URL
    #[serde(rename(serialize = "rtmpUrl", deserialize = "rtmpUrl"))]
    RtmpUrl(Box<crate::types::RtmpUrl>),
}

impl RtmpUrl {
    /// Convenience constructor to create a [`RtmpUrl::RtmpUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rtmp_url(val: crate::types::RtmpUrl) -> Self {
        Self::RtmpUrl(Box::new(val))
    }

}

/// Converts a [`crate::types::RtmpUrl`] into [`RtmpUrl`].
impl From<crate::types::RtmpUrl> for RtmpUrl {
    fn from(val: crate::types::RtmpUrl) -> Self {
        Self::RtmpUrl(Box::new(val))
    }
}

/// Contains settings for Firebase Authentication in the official applications
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FirebaseAuthenticationSettings {
    /// Settings for Firebase Authentication in the official Android application
    #[serde(rename(serialize = "firebaseAuthenticationSettingsAndroid", deserialize = "firebaseAuthenticationSettingsAndroid"))]
    Android,
    /// Settings for Firebase Authentication in the official iOS application
    #[serde(rename(serialize = "firebaseAuthenticationSettingsIos", deserialize = "firebaseAuthenticationSettingsIos"))]
    Ios(Box<crate::types::FirebaseAuthenticationSettingsIos>),
}

impl FirebaseAuthenticationSettings {
    /// Convenience constructor to create a [`FirebaseAuthenticationSettings::Ios`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ios(val: crate::types::FirebaseAuthenticationSettingsIos) -> Self {
        Self::Ios(Box::new(val))
    }

}

/// Converts a [`crate::types::FirebaseAuthenticationSettingsIos`] into [`FirebaseAuthenticationSettings`].
impl From<crate::types::FirebaseAuthenticationSettingsIos> for FirebaseAuthenticationSettings {
    fn from(val: crate::types::FirebaseAuthenticationSettingsIos) -> Self {
        Self::Ios(Box::new(val))
    }
}

/// TDLib `PhoneNumberAuthenticationSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberAuthenticationSettings {
    /// Contains settings for the authentication of the user's phone number
    #[serde(rename(serialize = "phoneNumberAuthenticationSettings", deserialize = "phoneNumberAuthenticationSettings"))]
    PhoneNumberAuthenticationSettings(Box<crate::types::PhoneNumberAuthenticationSettings>),
}

impl PhoneNumberAuthenticationSettings {
    /// Convenience constructor to create a [`PhoneNumberAuthenticationSettings::PhoneNumberAuthenticationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number_authentication_settings(val: crate::types::PhoneNumberAuthenticationSettings) -> Self {
        Self::PhoneNumberAuthenticationSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::PhoneNumberAuthenticationSettings`] into [`PhoneNumberAuthenticationSettings`].
impl From<crate::types::PhoneNumberAuthenticationSettings> for PhoneNumberAuthenticationSettings {
    fn from(val: crate::types::PhoneNumberAuthenticationSettings) -> Self {
        Self::PhoneNumberAuthenticationSettings(Box::new(val))
    }
}

/// Describes result of speech recognition in a voice note
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SpeechRecognitionResult {
    /// The speech recognition is ongoing
    #[serde(rename(serialize = "speechRecognitionResultPending", deserialize = "speechRecognitionResultPending"))]
    Pending(Box<crate::types::SpeechRecognitionResultPending>),
    /// The speech recognition successfully finished
    #[serde(rename(serialize = "speechRecognitionResultText", deserialize = "speechRecognitionResultText"))]
    Text(Box<crate::types::SpeechRecognitionResultText>),
    /// The speech recognition failed
    #[serde(rename(serialize = "speechRecognitionResultError", deserialize = "speechRecognitionResultError"))]
    Error(Box<crate::types::SpeechRecognitionResultError>),
}

impl SpeechRecognitionResult {
    /// Convenience constructor to create a [`SpeechRecognitionResult::Pending`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending(val: crate::types::SpeechRecognitionResultPending) -> Self {
        Self::Pending(Box::new(val))
    }

    /// Convenience constructor to create a [`SpeechRecognitionResult::Text`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text(val: crate::types::SpeechRecognitionResultText) -> Self {
        Self::Text(Box::new(val))
    }

    /// Convenience constructor to create a [`SpeechRecognitionResult::Error`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn error(val: crate::types::SpeechRecognitionResultError) -> Self {
        Self::Error(Box::new(val))
    }

}

/// Converts a [`crate::types::SpeechRecognitionResultPending`] into [`SpeechRecognitionResult`].
impl From<crate::types::SpeechRecognitionResultPending> for SpeechRecognitionResult {
    fn from(val: crate::types::SpeechRecognitionResultPending) -> Self {
        Self::Pending(Box::new(val))
    }
}

/// Converts a [`crate::types::SpeechRecognitionResultText`] into [`SpeechRecognitionResult`].
impl From<crate::types::SpeechRecognitionResultText> for SpeechRecognitionResult {
    fn from(val: crate::types::SpeechRecognitionResultText) -> Self {
        Self::Text(Box::new(val))
    }
}

/// Converts a [`crate::types::SpeechRecognitionResultError`] into [`SpeechRecognitionResult`].
impl From<crate::types::SpeechRecognitionResultError> for SpeechRecognitionResult {
    fn from(val: crate::types::SpeechRecognitionResultError) -> Self {
        Self::Error(Box::new(val))
    }
}

/// TDLib `HttpUrl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum HttpUrl {
    /// Contains an HTTP URL
    #[serde(rename(serialize = "httpUrl", deserialize = "httpUrl"))]
    HttpUrl(Box<crate::types::HttpUrl>),
}

impl HttpUrl {
    /// Convenience constructor to create a [`HttpUrl::HttpUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn http_url(val: crate::types::HttpUrl) -> Self {
        Self::HttpUrl(Box::new(val))
    }

}

/// Converts a [`crate::types::HttpUrl`] into [`HttpUrl`].
impl From<crate::types::HttpUrl> for HttpUrl {
    fn from(val: crate::types::HttpUrl) -> Self {
        Self::HttpUrl(Box::new(val))
    }
}

/// Represents a single result of an inline query; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputInlineQueryResult {
    /// Represents a link to an animated GIF or an animated (i.e., without sound) H.264/MPEG-4 AVC video
    #[serde(rename(serialize = "inputInlineQueryResultAnimation", deserialize = "inputInlineQueryResultAnimation"))]
    Animation(Box<crate::types::InputInlineQueryResultAnimation>),
    /// Represents a link to an article or web page
    #[serde(rename(serialize = "inputInlineQueryResultArticle", deserialize = "inputInlineQueryResultArticle"))]
    Article(Box<crate::types::InputInlineQueryResultArticle>),
    /// Represents a link to an MP3 audio file
    #[serde(rename(serialize = "inputInlineQueryResultAudio", deserialize = "inputInlineQueryResultAudio"))]
    Audio(Box<crate::types::InputInlineQueryResultAudio>),
    /// Represents a user contact
    #[serde(rename(serialize = "inputInlineQueryResultContact", deserialize = "inputInlineQueryResultContact"))]
    Contact(Box<crate::types::InputInlineQueryResultContact>),
    /// Represents a link to a file
    #[serde(rename(serialize = "inputInlineQueryResultDocument", deserialize = "inputInlineQueryResultDocument"))]
    Document(Box<crate::types::InputInlineQueryResultDocument>),
    /// Represents a game
    #[serde(rename(serialize = "inputInlineQueryResultGame", deserialize = "inputInlineQueryResultGame"))]
    Game(Box<crate::types::InputInlineQueryResultGame>),
    /// Represents a point on the map
    #[serde(rename(serialize = "inputInlineQueryResultLocation", deserialize = "inputInlineQueryResultLocation"))]
    Location(Box<crate::types::InputInlineQueryResultLocation>),
    /// Represents link to a JPEG image
    #[serde(rename(serialize = "inputInlineQueryResultPhoto", deserialize = "inputInlineQueryResultPhoto"))]
    Photo(Box<crate::types::InputInlineQueryResultPhoto>),
    /// Represents a link to a WEBP, TGS, or WEBM sticker
    #[serde(rename(serialize = "inputInlineQueryResultSticker", deserialize = "inputInlineQueryResultSticker"))]
    Sticker(Box<crate::types::InputInlineQueryResultSticker>),
    /// Represents information about a venue
    #[serde(rename(serialize = "inputInlineQueryResultVenue", deserialize = "inputInlineQueryResultVenue"))]
    Venue(Box<crate::types::InputInlineQueryResultVenue>),
    /// Represents a link to a page containing an embedded video player or a video file
    #[serde(rename(serialize = "inputInlineQueryResultVideo", deserialize = "inputInlineQueryResultVideo"))]
    Video(Box<crate::types::InputInlineQueryResultVideo>),
    /// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
    #[serde(rename(serialize = "inputInlineQueryResultVoiceNote", deserialize = "inputInlineQueryResultVoiceNote"))]
    VoiceNote(Box<crate::types::InputInlineQueryResultVoiceNote>),
}

impl InputInlineQueryResult {
    /// Convenience constructor to create a [`InputInlineQueryResult::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::InputInlineQueryResultAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Article`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn article(val: crate::types::InputInlineQueryResultArticle) -> Self {
        Self::Article(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::InputInlineQueryResultAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Contact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact(val: crate::types::InputInlineQueryResultContact) -> Self {
        Self::Contact(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::InputInlineQueryResultDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::InputInlineQueryResultGame) -> Self {
        Self::Game(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::InputInlineQueryResultLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InputInlineQueryResultPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::InputInlineQueryResultSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::InputInlineQueryResultVenue) -> Self {
        Self::Venue(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InputInlineQueryResultVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`InputInlineQueryResult::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::InputInlineQueryResultVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

}

/// Converts a [`crate::types::InputInlineQueryResultAnimation`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultAnimation> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultArticle`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultArticle> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultArticle) -> Self {
        Self::Article(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultAudio`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultAudio> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultContact`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultContact> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultContact) -> Self {
        Self::Contact(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultDocument`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultDocument> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultGame`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultGame> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultGame) -> Self {
        Self::Game(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultLocation`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultLocation> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultPhoto`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultPhoto> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultSticker`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultSticker> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultVenue`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultVenue> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultVenue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultVideo`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultVideo> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::InputInlineQueryResultVoiceNote`] into [`InputInlineQueryResult`].
impl From<crate::types::InputInlineQueryResultVoiceNote> for InputInlineQueryResult {
    fn from(val: crate::types::InputInlineQueryResultVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Represents a single result of an inline query
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResult {
    /// Represents a link to an article or web page
    #[serde(rename(serialize = "inlineQueryResultArticle", deserialize = "inlineQueryResultArticle"))]
    Article(Box<crate::types::InlineQueryResultArticle>),
    /// Represents a user contact
    #[serde(rename(serialize = "inlineQueryResultContact", deserialize = "inlineQueryResultContact"))]
    Contact(Box<crate::types::InlineQueryResultContact>),
    /// Represents a point on the map
    #[serde(rename(serialize = "inlineQueryResultLocation", deserialize = "inlineQueryResultLocation"))]
    Location(Box<crate::types::InlineQueryResultLocation>),
    /// Represents information about a venue
    #[serde(rename(serialize = "inlineQueryResultVenue", deserialize = "inlineQueryResultVenue"))]
    Venue(Box<crate::types::InlineQueryResultVenue>),
    /// Represents information about a game
    #[serde(rename(serialize = "inlineQueryResultGame", deserialize = "inlineQueryResultGame"))]
    Game(Box<crate::types::InlineQueryResultGame>),
    /// Represents an animation file
    #[serde(rename(serialize = "inlineQueryResultAnimation", deserialize = "inlineQueryResultAnimation"))]
    Animation(Box<crate::types::InlineQueryResultAnimation>),
    /// Represents an audio file
    #[serde(rename(serialize = "inlineQueryResultAudio", deserialize = "inlineQueryResultAudio"))]
    Audio(Box<crate::types::InlineQueryResultAudio>),
    /// Represents a document
    #[serde(rename(serialize = "inlineQueryResultDocument", deserialize = "inlineQueryResultDocument"))]
    Document(Box<crate::types::InlineQueryResultDocument>),
    /// Represents a photo
    #[serde(rename(serialize = "inlineQueryResultPhoto", deserialize = "inlineQueryResultPhoto"))]
    Photo(Box<crate::types::InlineQueryResultPhoto>),
    /// Represents a sticker
    #[serde(rename(serialize = "inlineQueryResultSticker", deserialize = "inlineQueryResultSticker"))]
    Sticker(Box<crate::types::InlineQueryResultSticker>),
    /// Represents a video
    #[serde(rename(serialize = "inlineQueryResultVideo", deserialize = "inlineQueryResultVideo"))]
    Video(Box<crate::types::InlineQueryResultVideo>),
    /// Represents a voice note
    #[serde(rename(serialize = "inlineQueryResultVoiceNote", deserialize = "inlineQueryResultVoiceNote"))]
    VoiceNote(Box<crate::types::InlineQueryResultVoiceNote>),
}

impl InlineQueryResult {
    /// Convenience constructor to create a [`InlineQueryResult::Article`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn article(val: crate::types::InlineQueryResultArticle) -> Self {
        Self::Article(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Contact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact(val: crate::types::InlineQueryResultContact) -> Self {
        Self::Contact(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::InlineQueryResultLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::InlineQueryResultVenue) -> Self {
        Self::Venue(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::InlineQueryResultGame) -> Self {
        Self::Game(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::InlineQueryResultAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::InlineQueryResultAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::InlineQueryResultDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InlineQueryResultPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::InlineQueryResultSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InlineQueryResultVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResult::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::InlineQueryResultVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineQueryResultArticle`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultArticle> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultArticle) -> Self {
        Self::Article(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultContact`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultContact> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultContact) -> Self {
        Self::Contact(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultLocation`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultLocation> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultVenue`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultVenue> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVenue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultGame`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultGame> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultGame) -> Self {
        Self::Game(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultAnimation`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultAnimation> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultAudio`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultAudio> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultDocument`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultDocument> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultPhoto`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultPhoto> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultSticker`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultSticker> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultVideo`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultVideo> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultVoiceNote`] into [`InlineQueryResult`].
impl From<crate::types::InlineQueryResultVoiceNote> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Represents type of button in results of inline query
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResultsButtonType {
    /// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
    #[serde(rename(serialize = "inlineQueryResultsButtonTypeStartBot", deserialize = "inlineQueryResultsButtonTypeStartBot"))]
    StartBot(Box<crate::types::InlineQueryResultsButtonTypeStartBot>),
    /// Describes the button that opens a Web App by calling getWebAppUrl
    #[serde(rename(serialize = "inlineQueryResultsButtonTypeWebApp", deserialize = "inlineQueryResultsButtonTypeWebApp"))]
    WebApp(Box<crate::types::InlineQueryResultsButtonTypeWebApp>),
}

impl InlineQueryResultsButtonType {
    /// Convenience constructor to create a [`InlineQueryResultsButtonType::StartBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn start_bot(val: crate::types::InlineQueryResultsButtonTypeStartBot) -> Self {
        Self::StartBot(Box::new(val))
    }

    /// Convenience constructor to create a [`InlineQueryResultsButtonType::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::InlineQueryResultsButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineQueryResultsButtonTypeStartBot`] into [`InlineQueryResultsButtonType`].
impl From<crate::types::InlineQueryResultsButtonTypeStartBot> for InlineQueryResultsButtonType {
    fn from(val: crate::types::InlineQueryResultsButtonTypeStartBot) -> Self {
        Self::StartBot(Box::new(val))
    }
}

/// Converts a [`crate::types::InlineQueryResultsButtonTypeWebApp`] into [`InlineQueryResultsButtonType`].
impl From<crate::types::InlineQueryResultsButtonTypeWebApp> for InlineQueryResultsButtonType {
    fn from(val: crate::types::InlineQueryResultsButtonTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// TDLib `InlineQueryResultsButton` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResultsButton {
    /// Represents a button to be shown above inline query results
    #[serde(rename(serialize = "inlineQueryResultsButton", deserialize = "inlineQueryResultsButton"))]
    InlineQueryResultsButton(Box<crate::types::InlineQueryResultsButton>),
}

impl InlineQueryResultsButton {
    /// Convenience constructor to create a [`InlineQueryResultsButton::InlineQueryResultsButton`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_query_results_button(val: crate::types::InlineQueryResultsButton) -> Self {
        Self::InlineQueryResultsButton(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineQueryResultsButton`] into [`InlineQueryResultsButton`].
impl From<crate::types::InlineQueryResultsButton> for InlineQueryResultsButton {
    fn from(val: crate::types::InlineQueryResultsButton) -> Self {
        Self::InlineQueryResultsButton(Box::new(val))
    }
}

/// TDLib `InlineQueryResults` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResults {
    /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
    #[serde(rename(serialize = "inlineQueryResults", deserialize = "inlineQueryResults"))]
    InlineQueryResults(Box<crate::types::InlineQueryResults>),
}

impl InlineQueryResults {
    /// Convenience constructor to create a [`InlineQueryResults::InlineQueryResults`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_query_results(val: crate::types::InlineQueryResults) -> Self {
        Self::InlineQueryResults(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineQueryResults`] into [`InlineQueryResults`].
impl From<crate::types::InlineQueryResults> for InlineQueryResults {
    fn from(val: crate::types::InlineQueryResults) -> Self {
        Self::InlineQueryResults(Box::new(val))
    }
}

/// TDLib `CustomRequestResult` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CustomRequestResult {
    /// Contains the result of a custom request
    #[serde(rename(serialize = "customRequestResult", deserialize = "customRequestResult"))]
    CustomRequestResult(Box<crate::types::CustomRequestResult>),
}

impl CustomRequestResult {
    /// Convenience constructor to create a [`CustomRequestResult::CustomRequestResult`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_request_result(val: crate::types::CustomRequestResult) -> Self {
        Self::CustomRequestResult(Box::new(val))
    }

}

/// Converts a [`crate::types::CustomRequestResult`] into [`CustomRequestResult`].
impl From<crate::types::CustomRequestResult> for CustomRequestResult {
    fn from(val: crate::types::CustomRequestResult) -> Self {
        Self::CustomRequestResult(Box::new(val))
    }
}

/// TDLib `GameHighScore` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GameHighScore {
    /// Contains one row of the game high score table
    #[serde(rename(serialize = "gameHighScore", deserialize = "gameHighScore"))]
    GameHighScore(Box<crate::types::GameHighScore>),
}

impl GameHighScore {
    /// Convenience constructor to create a [`GameHighScore::GameHighScore`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game_high_score(val: crate::types::GameHighScore) -> Self {
        Self::GameHighScore(Box::new(val))
    }

}

/// Converts a [`crate::types::GameHighScore`] into [`GameHighScore`].
impl From<crate::types::GameHighScore> for GameHighScore {
    fn from(val: crate::types::GameHighScore) -> Self {
        Self::GameHighScore(Box::new(val))
    }
}

/// TDLib `GameHighScores` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GameHighScores {
    /// Contains a list of game high scores
    #[serde(rename(serialize = "gameHighScores", deserialize = "gameHighScores"))]
    GameHighScores(Box<crate::types::GameHighScores>),
}

impl GameHighScores {
    /// Convenience constructor to create a [`GameHighScores::GameHighScores`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game_high_scores(val: crate::types::GameHighScores) -> Self {
        Self::GameHighScores(Box::new(val))
    }

}

/// Converts a [`crate::types::GameHighScores`] into [`GameHighScores`].
impl From<crate::types::GameHighScores> for GameHighScores {
    fn from(val: crate::types::GameHighScores) -> Self {
        Self::GameHighScores(Box::new(val))
    }
}

/// Represents the value of a string in a language pack
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStringValue {
    /// An ordinary language pack string
    #[serde(rename(serialize = "languagePackStringValueOrdinary", deserialize = "languagePackStringValueOrdinary"))]
    Ordinary(Box<crate::types::LanguagePackStringValueOrdinary>),
    /// A language pack string which has different forms based on the number of some object it mentions. See https:www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more information
    #[serde(rename(serialize = "languagePackStringValuePluralized", deserialize = "languagePackStringValuePluralized"))]
    Pluralized(Box<crate::types::LanguagePackStringValuePluralized>),
    /// A deleted language pack string, the value must be taken from the built-in English language pack
    #[serde(rename(serialize = "languagePackStringValueDeleted", deserialize = "languagePackStringValueDeleted"))]
    Deleted,
}

impl LanguagePackStringValue {
    /// Convenience constructor to create a [`LanguagePackStringValue::Ordinary`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ordinary(val: crate::types::LanguagePackStringValueOrdinary) -> Self {
        Self::Ordinary(Box::new(val))
    }

    /// Convenience constructor to create a [`LanguagePackStringValue::Pluralized`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pluralized(val: crate::types::LanguagePackStringValuePluralized) -> Self {
        Self::Pluralized(Box::new(val))
    }

}

/// Converts a [`crate::types::LanguagePackStringValueOrdinary`] into [`LanguagePackStringValue`].
impl From<crate::types::LanguagePackStringValueOrdinary> for LanguagePackStringValue {
    fn from(val: crate::types::LanguagePackStringValueOrdinary) -> Self {
        Self::Ordinary(Box::new(val))
    }
}

/// Converts a [`crate::types::LanguagePackStringValuePluralized`] into [`LanguagePackStringValue`].
impl From<crate::types::LanguagePackStringValuePluralized> for LanguagePackStringValue {
    fn from(val: crate::types::LanguagePackStringValuePluralized) -> Self {
        Self::Pluralized(Box::new(val))
    }
}

/// TDLib `LanguagePackString` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackString {
    /// Represents one language pack string
    #[serde(rename(serialize = "languagePackString", deserialize = "languagePackString"))]
    LanguagePackString(Box<crate::types::LanguagePackString>),
}

impl LanguagePackString {
    /// Convenience constructor to create a [`LanguagePackString::LanguagePackString`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language_pack_string(val: crate::types::LanguagePackString) -> Self {
        Self::LanguagePackString(Box::new(val))
    }

}

/// Converts a [`crate::types::LanguagePackString`] into [`LanguagePackString`].
impl From<crate::types::LanguagePackString> for LanguagePackString {
    fn from(val: crate::types::LanguagePackString) -> Self {
        Self::LanguagePackString(Box::new(val))
    }
}

/// TDLib `LanguagePackStrings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackStrings {
    /// Contains a list of language pack strings
    #[serde(rename(serialize = "languagePackStrings", deserialize = "languagePackStrings"))]
    LanguagePackStrings(Box<crate::types::LanguagePackStrings>),
}

impl LanguagePackStrings {
    /// Convenience constructor to create a [`LanguagePackStrings::LanguagePackStrings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language_pack_strings(val: crate::types::LanguagePackStrings) -> Self {
        Self::LanguagePackStrings(Box::new(val))
    }

}

/// Converts a [`crate::types::LanguagePackStrings`] into [`LanguagePackStrings`].
impl From<crate::types::LanguagePackStrings> for LanguagePackStrings {
    fn from(val: crate::types::LanguagePackStrings) -> Self {
        Self::LanguagePackStrings(Box::new(val))
    }
}

/// TDLib `LanguagePackInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LanguagePackInfo {
    /// Contains information about a language pack
    #[serde(rename(serialize = "languagePackInfo", deserialize = "languagePackInfo"))]
    LanguagePackInfo(Box<crate::types::LanguagePackInfo>),
}

impl LanguagePackInfo {
    /// Convenience constructor to create a [`LanguagePackInfo::LanguagePackInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language_pack_info(val: crate::types::LanguagePackInfo) -> Self {
        Self::LanguagePackInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::LanguagePackInfo`] into [`LanguagePackInfo`].
impl From<crate::types::LanguagePackInfo> for LanguagePackInfo {
    fn from(val: crate::types::LanguagePackInfo) -> Self {
        Self::LanguagePackInfo(Box::new(val))
    }
}

/// TDLib `LocalizationTargetInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LocalizationTargetInfo {
    /// Contains information about the current localization target
    #[serde(rename(serialize = "localizationTargetInfo", deserialize = "localizationTargetInfo"))]
    LocalizationTargetInfo(Box<crate::types::LocalizationTargetInfo>),
}

impl LocalizationTargetInfo {
    /// Convenience constructor to create a [`LocalizationTargetInfo::LocalizationTargetInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn localization_target_info(val: crate::types::LocalizationTargetInfo) -> Self {
        Self::LocalizationTargetInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::LocalizationTargetInfo`] into [`LocalizationTargetInfo`].
impl From<crate::types::LocalizationTargetInfo> for LocalizationTargetInfo {
    fn from(val: crate::types::LocalizationTargetInfo) -> Self {
        Self::LocalizationTargetInfo(Box::new(val))
    }
}

/// Describes an in-store transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoreTransaction {
    /// A purchase through App Store
    #[serde(rename(serialize = "storeTransactionAppStore", deserialize = "storeTransactionAppStore"))]
    AppStore(Box<crate::types::StoreTransactionAppStore>),
    /// A purchase through Google Play
    #[serde(rename(serialize = "storeTransactionGooglePlay", deserialize = "storeTransactionGooglePlay"))]
    GooglePlay(Box<crate::types::StoreTransactionGooglePlay>),
}

impl StoreTransaction {
    /// Convenience constructor to create a [`StoreTransaction::AppStore`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn app_store(val: crate::types::StoreTransactionAppStore) -> Self {
        Self::AppStore(Box::new(val))
    }

    /// Convenience constructor to create a [`StoreTransaction::GooglePlay`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn google_play(val: crate::types::StoreTransactionGooglePlay) -> Self {
        Self::GooglePlay(Box::new(val))
    }

}

/// Converts a [`crate::types::StoreTransactionAppStore`] into [`StoreTransaction`].
impl From<crate::types::StoreTransactionAppStore> for StoreTransaction {
    fn from(val: crate::types::StoreTransactionAppStore) -> Self {
        Self::AppStore(Box::new(val))
    }
}

/// Converts a [`crate::types::StoreTransactionGooglePlay`] into [`StoreTransaction`].
impl From<crate::types::StoreTransactionGooglePlay> for StoreTransaction {
    fn from(val: crate::types::StoreTransactionGooglePlay) -> Self {
        Self::GooglePlay(Box::new(val))
    }
}

/// Represents a data needed to subscribe for push notifications through registerDevice method.
/// To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https:my.telegram.org
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DeviceToken {
    /// A token for Firebase Cloud Messaging
    #[serde(rename(serialize = "deviceTokenFirebaseCloudMessaging", deserialize = "deviceTokenFirebaseCloudMessaging"))]
    FirebaseCloudMessaging(Box<crate::types::DeviceTokenFirebaseCloudMessaging>),
    /// A token for Apple Push Notification service
    #[serde(rename(serialize = "deviceTokenApplePush", deserialize = "deviceTokenApplePush"))]
    ApplePush(Box<crate::types::DeviceTokenApplePush>),
    /// A token for Apple Push Notification service VoIP notifications
    #[serde(rename(serialize = "deviceTokenApplePushVoIP", deserialize = "deviceTokenApplePushVoIP"))]
    ApplePushVoIp(Box<crate::types::DeviceTokenApplePushVoIp>),
    /// A token for Windows Push Notification Services
    #[serde(rename(serialize = "deviceTokenWindowsPush", deserialize = "deviceTokenWindowsPush"))]
    WindowsPush(Box<crate::types::DeviceTokenWindowsPush>),
    /// A token for Microsoft Push Notification Service
    #[serde(rename(serialize = "deviceTokenMicrosoftPush", deserialize = "deviceTokenMicrosoftPush"))]
    MicrosoftPush(Box<crate::types::DeviceTokenMicrosoftPush>),
    /// A token for Microsoft Push Notification Service VoIP channel
    #[serde(rename(serialize = "deviceTokenMicrosoftPushVoIP", deserialize = "deviceTokenMicrosoftPushVoIP"))]
    MicrosoftPushVoIp(Box<crate::types::DeviceTokenMicrosoftPushVoIp>),
    /// A token for web Push API
    #[serde(rename(serialize = "deviceTokenWebPush", deserialize = "deviceTokenWebPush"))]
    WebPush(Box<crate::types::DeviceTokenWebPush>),
    /// A token for Simple Push API for Firefox OS
    #[serde(rename(serialize = "deviceTokenSimplePush", deserialize = "deviceTokenSimplePush"))]
    SimplePush(Box<crate::types::DeviceTokenSimplePush>),
    /// A token for Ubuntu Push Client service
    #[serde(rename(serialize = "deviceTokenUbuntuPush", deserialize = "deviceTokenUbuntuPush"))]
    UbuntuPush(Box<crate::types::DeviceTokenUbuntuPush>),
    /// A token for BlackBerry Push Service
    #[serde(rename(serialize = "deviceTokenBlackBerryPush", deserialize = "deviceTokenBlackBerryPush"))]
    BlackBerryPush(Box<crate::types::DeviceTokenBlackBerryPush>),
    /// A token for Tizen Push Service
    #[serde(rename(serialize = "deviceTokenTizenPush", deserialize = "deviceTokenTizenPush"))]
    TizenPush(Box<crate::types::DeviceTokenTizenPush>),
    /// A token for HUAWEI Push Service
    #[serde(rename(serialize = "deviceTokenHuaweiPush", deserialize = "deviceTokenHuaweiPush"))]
    HuaweiPush(Box<crate::types::DeviceTokenHuaweiPush>),
}

impl DeviceToken {
    /// Convenience constructor to create a [`DeviceToken::FirebaseCloudMessaging`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn firebase_cloud_messaging(val: crate::types::DeviceTokenFirebaseCloudMessaging) -> Self {
        Self::FirebaseCloudMessaging(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::ApplePush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn apple_push(val: crate::types::DeviceTokenApplePush) -> Self {
        Self::ApplePush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::ApplePushVoIp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn apple_push_vo_ip(val: crate::types::DeviceTokenApplePushVoIp) -> Self {
        Self::ApplePushVoIp(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::WindowsPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn windows_push(val: crate::types::DeviceTokenWindowsPush) -> Self {
        Self::WindowsPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::MicrosoftPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn microsoft_push(val: crate::types::DeviceTokenMicrosoftPush) -> Self {
        Self::MicrosoftPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::MicrosoftPushVoIp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn microsoft_push_vo_ip(val: crate::types::DeviceTokenMicrosoftPushVoIp) -> Self {
        Self::MicrosoftPushVoIp(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::WebPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_push(val: crate::types::DeviceTokenWebPush) -> Self {
        Self::WebPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::SimplePush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn simple_push(val: crate::types::DeviceTokenSimplePush) -> Self {
        Self::SimplePush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::UbuntuPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ubuntu_push(val: crate::types::DeviceTokenUbuntuPush) -> Self {
        Self::UbuntuPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::BlackBerryPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn black_berry_push(val: crate::types::DeviceTokenBlackBerryPush) -> Self {
        Self::BlackBerryPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::TizenPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn tizen_push(val: crate::types::DeviceTokenTizenPush) -> Self {
        Self::TizenPush(Box::new(val))
    }

    /// Convenience constructor to create a [`DeviceToken::HuaweiPush`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn huawei_push(val: crate::types::DeviceTokenHuaweiPush) -> Self {
        Self::HuaweiPush(Box::new(val))
    }

}

/// Converts a [`crate::types::DeviceTokenFirebaseCloudMessaging`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenFirebaseCloudMessaging> for DeviceToken {
    fn from(val: crate::types::DeviceTokenFirebaseCloudMessaging) -> Self {
        Self::FirebaseCloudMessaging(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenApplePush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenApplePush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenApplePush) -> Self {
        Self::ApplePush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenApplePushVoIp`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenApplePushVoIp> for DeviceToken {
    fn from(val: crate::types::DeviceTokenApplePushVoIp) -> Self {
        Self::ApplePushVoIp(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenWindowsPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenWindowsPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenWindowsPush) -> Self {
        Self::WindowsPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenMicrosoftPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenMicrosoftPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenMicrosoftPush) -> Self {
        Self::MicrosoftPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenMicrosoftPushVoIp`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenMicrosoftPushVoIp> for DeviceToken {
    fn from(val: crate::types::DeviceTokenMicrosoftPushVoIp) -> Self {
        Self::MicrosoftPushVoIp(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenWebPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenWebPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenWebPush) -> Self {
        Self::WebPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenSimplePush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenSimplePush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenSimplePush) -> Self {
        Self::SimplePush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenUbuntuPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenUbuntuPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenUbuntuPush) -> Self {
        Self::UbuntuPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenBlackBerryPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenBlackBerryPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenBlackBerryPush) -> Self {
        Self::BlackBerryPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenTizenPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenTizenPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenTizenPush) -> Self {
        Self::TizenPush(Box::new(val))
    }
}

/// Converts a [`crate::types::DeviceTokenHuaweiPush`] into [`DeviceToken`].
impl From<crate::types::DeviceTokenHuaweiPush> for DeviceToken {
    fn from(val: crate::types::DeviceTokenHuaweiPush) -> Self {
        Self::HuaweiPush(Box::new(val))
    }
}

/// TDLib `PushReceiverId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PushReceiverId {
    /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
    #[serde(rename(serialize = "pushReceiverId", deserialize = "pushReceiverId"))]
    PushReceiverId(Box<crate::types::PushReceiverId>),
}

impl PushReceiverId {
    /// Convenience constructor to create a [`PushReceiverId::PushReceiverId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn push_receiver_id(val: crate::types::PushReceiverId) -> Self {
        Self::PushReceiverId(Box::new(val))
    }

}

/// Converts a [`crate::types::PushReceiverId`] into [`PushReceiverId`].
impl From<crate::types::PushReceiverId> for PushReceiverId {
    fn from(val: crate::types::PushReceiverId) -> Self {
        Self::PushReceiverId(Box::new(val))
    }
}

/// Describes a fill of a background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundFill {
    /// Describes a solid fill of a background
    #[serde(rename(serialize = "backgroundFillSolid", deserialize = "backgroundFillSolid"))]
    Solid(Box<crate::types::BackgroundFillSolid>),
    /// Describes a gradient fill of a background
    #[serde(rename(serialize = "backgroundFillGradient", deserialize = "backgroundFillGradient"))]
    Gradient(Box<crate::types::BackgroundFillGradient>),
    /// Describes a freeform gradient fill of a background
    #[serde(rename(serialize = "backgroundFillFreeformGradient", deserialize = "backgroundFillFreeformGradient"))]
    FreeformGradient(Box<crate::types::BackgroundFillFreeformGradient>),
}

impl BackgroundFill {
    /// Convenience constructor to create a [`BackgroundFill::Solid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn solid(val: crate::types::BackgroundFillSolid) -> Self {
        Self::Solid(Box::new(val))
    }

    /// Convenience constructor to create a [`BackgroundFill::Gradient`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gradient(val: crate::types::BackgroundFillGradient) -> Self {
        Self::Gradient(Box::new(val))
    }

    /// Convenience constructor to create a [`BackgroundFill::FreeformGradient`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn freeform_gradient(val: crate::types::BackgroundFillFreeformGradient) -> Self {
        Self::FreeformGradient(Box::new(val))
    }

}

/// Converts a [`crate::types::BackgroundFillSolid`] into [`BackgroundFill`].
impl From<crate::types::BackgroundFillSolid> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillSolid) -> Self {
        Self::Solid(Box::new(val))
    }
}

/// Converts a [`crate::types::BackgroundFillGradient`] into [`BackgroundFill`].
impl From<crate::types::BackgroundFillGradient> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillGradient) -> Self {
        Self::Gradient(Box::new(val))
    }
}

/// Converts a [`crate::types::BackgroundFillFreeformGradient`] into [`BackgroundFill`].
impl From<crate::types::BackgroundFillFreeformGradient> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillFreeformGradient) -> Self {
        Self::FreeformGradient(Box::new(val))
    }
}

/// Describes the type of background
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BackgroundType {
    /// A wallpaper in JPEG format
    #[serde(rename(serialize = "backgroundTypeWallpaper", deserialize = "backgroundTypeWallpaper"))]
    Wallpaper(Box<crate::types::BackgroundTypeWallpaper>),
    /// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
    #[serde(rename(serialize = "backgroundTypePattern", deserialize = "backgroundTypePattern"))]
    Pattern(Box<crate::types::BackgroundTypePattern>),
    /// A filled background
    #[serde(rename(serialize = "backgroundTypeFill", deserialize = "backgroundTypeFill"))]
    Fill(Box<crate::types::BackgroundTypeFill>),
    /// A background from a chat theme based on an emoji; can be used only as a chat background in channels
    #[serde(rename(serialize = "backgroundTypeChatTheme", deserialize = "backgroundTypeChatTheme"))]
    ChatTheme(Box<crate::types::BackgroundTypeChatTheme>),
}

impl BackgroundType {
    /// Convenience constructor to create a [`BackgroundType::Wallpaper`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn wallpaper(val: crate::types::BackgroundTypeWallpaper) -> Self {
        Self::Wallpaper(Box::new(val))
    }

    /// Convenience constructor to create a [`BackgroundType::Pattern`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pattern(val: crate::types::BackgroundTypePattern) -> Self {
        Self::Pattern(Box::new(val))
    }

    /// Convenience constructor to create a [`BackgroundType::Fill`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fill(val: crate::types::BackgroundTypeFill) -> Self {
        Self::Fill(Box::new(val))
    }

    /// Convenience constructor to create a [`BackgroundType::ChatTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_theme(val: crate::types::BackgroundTypeChatTheme) -> Self {
        Self::ChatTheme(Box::new(val))
    }

}

/// Converts a [`crate::types::BackgroundTypeWallpaper`] into [`BackgroundType`].
impl From<crate::types::BackgroundTypeWallpaper> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeWallpaper) -> Self {
        Self::Wallpaper(Box::new(val))
    }
}

/// Converts a [`crate::types::BackgroundTypePattern`] into [`BackgroundType`].
impl From<crate::types::BackgroundTypePattern> for BackgroundType {
    fn from(val: crate::types::BackgroundTypePattern) -> Self {
        Self::Pattern(Box::new(val))
    }
}

/// Converts a [`crate::types::BackgroundTypeFill`] into [`BackgroundType`].
impl From<crate::types::BackgroundTypeFill> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeFill) -> Self {
        Self::Fill(Box::new(val))
    }
}

/// Converts a [`crate::types::BackgroundTypeChatTheme`] into [`BackgroundType`].
impl From<crate::types::BackgroundTypeChatTheme> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeChatTheme) -> Self {
        Self::ChatTheme(Box::new(val))
    }
}

/// Contains information about background to set
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBackground {
    /// A background from a local file
    #[serde(rename(serialize = "inputBackgroundLocal", deserialize = "inputBackgroundLocal"))]
    Local(Box<crate::types::InputBackgroundLocal>),
    /// A background from the server
    #[serde(rename(serialize = "inputBackgroundRemote", deserialize = "inputBackgroundRemote"))]
    Remote(Box<crate::types::InputBackgroundRemote>),
    /// A background previously set in the chat; for chat backgrounds only
    #[serde(rename(serialize = "inputBackgroundPrevious", deserialize = "inputBackgroundPrevious"))]
    Previous(Box<crate::types::InputBackgroundPrevious>),
}

impl InputBackground {
    /// Convenience constructor to create a [`InputBackground::Local`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn local(val: crate::types::InputBackgroundLocal) -> Self {
        Self::Local(Box::new(val))
    }

    /// Convenience constructor to create a [`InputBackground::Remote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn remote(val: crate::types::InputBackgroundRemote) -> Self {
        Self::Remote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputBackground::Previous`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn previous(val: crate::types::InputBackgroundPrevious) -> Self {
        Self::Previous(Box::new(val))
    }

}

/// Converts a [`crate::types::InputBackgroundLocal`] into [`InputBackground`].
impl From<crate::types::InputBackgroundLocal> for InputBackground {
    fn from(val: crate::types::InputBackgroundLocal) -> Self {
        Self::Local(Box::new(val))
    }
}

/// Converts a [`crate::types::InputBackgroundRemote`] into [`InputBackground`].
impl From<crate::types::InputBackgroundRemote> for InputBackground {
    fn from(val: crate::types::InputBackgroundRemote) -> Self {
        Self::Remote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputBackgroundPrevious`] into [`InputBackground`].
impl From<crate::types::InputBackgroundPrevious> for InputBackground {
    fn from(val: crate::types::InputBackgroundPrevious) -> Self {
        Self::Previous(Box::new(val))
    }
}

/// TDLib `TimeZone` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TimeZone {
    /// Describes a time zone
    #[serde(rename(serialize = "timeZone", deserialize = "timeZone"))]
    TimeZone(Box<crate::types::TimeZone>),
}

impl TimeZone {
    /// Convenience constructor to create a [`TimeZone::TimeZone`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn time_zone(val: crate::types::TimeZone) -> Self {
        Self::TimeZone(Box::new(val))
    }

}

/// Converts a [`crate::types::TimeZone`] into [`TimeZone`].
impl From<crate::types::TimeZone> for TimeZone {
    fn from(val: crate::types::TimeZone) -> Self {
        Self::TimeZone(Box::new(val))
    }
}

/// TDLib `TimeZones` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TimeZones {
    /// Contains a list of time zones
    #[serde(rename(serialize = "timeZones", deserialize = "timeZones"))]
    TimeZones(Box<crate::types::TimeZones>),
}

impl TimeZones {
    /// Convenience constructor to create a [`TimeZones::TimeZones`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn time_zones(val: crate::types::TimeZones) -> Self {
        Self::TimeZones(Box::new(val))
    }

}

/// Converts a [`crate::types::TimeZones`] into [`TimeZones`].
impl From<crate::types::TimeZones> for TimeZones {
    fn from(val: crate::types::TimeZones) -> Self {
        Self::TimeZones(Box::new(val))
    }
}

/// TDLib `Hashtags` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Hashtags {
    /// Contains a list of hashtags
    #[serde(rename(serialize = "hashtags", deserialize = "hashtags"))]
    Hashtags(Box<crate::types::Hashtags>),
}

impl Hashtags {
    /// Convenience constructor to create a [`Hashtags::Hashtags`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn hashtags(val: crate::types::Hashtags) -> Self {
        Self::Hashtags(Box::new(val))
    }

}

/// Converts a [`crate::types::Hashtags`] into [`Hashtags`].
impl From<crate::types::Hashtags> for Hashtags {
    fn from(val: crate::types::Hashtags) -> Self {
        Self::Hashtags(Box::new(val))
    }
}

/// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanTransferOwnershipResult {
    /// The session can be used
    #[serde(rename(serialize = "canTransferOwnershipResultOk", deserialize = "canTransferOwnershipResultOk"))]
    Ok,
    /// The 2-step verification needs to be enabled first
    #[serde(rename(serialize = "canTransferOwnershipResultPasswordNeeded", deserialize = "canTransferOwnershipResultPasswordNeeded"))]
    PasswordNeeded,
    /// The 2-step verification was enabled recently, user needs to wait
    #[serde(rename(serialize = "canTransferOwnershipResultPasswordTooFresh", deserialize = "canTransferOwnershipResultPasswordTooFresh"))]
    PasswordTooFresh(Box<crate::types::CanTransferOwnershipResultPasswordTooFresh>),
    /// The session was created recently, user needs to wait
    #[serde(rename(serialize = "canTransferOwnershipResultSessionTooFresh", deserialize = "canTransferOwnershipResultSessionTooFresh"))]
    SessionTooFresh(Box<crate::types::CanTransferOwnershipResultSessionTooFresh>),
}

impl CanTransferOwnershipResult {
    /// Convenience constructor to create a [`CanTransferOwnershipResult::PasswordTooFresh`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn password_too_fresh(val: crate::types::CanTransferOwnershipResultPasswordTooFresh) -> Self {
        Self::PasswordTooFresh(Box::new(val))
    }

    /// Convenience constructor to create a [`CanTransferOwnershipResult::SessionTooFresh`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn session_too_fresh(val: crate::types::CanTransferOwnershipResultSessionTooFresh) -> Self {
        Self::SessionTooFresh(Box::new(val))
    }

}

/// Converts a [`crate::types::CanTransferOwnershipResultPasswordTooFresh`] into [`CanTransferOwnershipResult`].
impl From<crate::types::CanTransferOwnershipResultPasswordTooFresh> for CanTransferOwnershipResult {
    fn from(val: crate::types::CanTransferOwnershipResultPasswordTooFresh) -> Self {
        Self::PasswordTooFresh(Box::new(val))
    }
}

/// Converts a [`crate::types::CanTransferOwnershipResultSessionTooFresh`] into [`CanTransferOwnershipResult`].
impl From<crate::types::CanTransferOwnershipResultSessionTooFresh> for CanTransferOwnershipResult {
    fn from(val: crate::types::CanTransferOwnershipResultSessionTooFresh) -> Self {
        Self::SessionTooFresh(Box::new(val))
    }
}

/// Represents result of 2-step verification password reset
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ResetPasswordResult {
    /// The password was reset
    #[serde(rename(serialize = "resetPasswordResultOk", deserialize = "resetPasswordResultOk"))]
    Ok,
    /// The password reset request is pending
    #[serde(rename(serialize = "resetPasswordResultPending", deserialize = "resetPasswordResultPending"))]
    Pending(Box<crate::types::ResetPasswordResultPending>),
    /// The password reset request was declined
    #[serde(rename(serialize = "resetPasswordResultDeclined", deserialize = "resetPasswordResultDeclined"))]
    Declined(Box<crate::types::ResetPasswordResultDeclined>),
}

impl ResetPasswordResult {
    /// Convenience constructor to create a [`ResetPasswordResult::Pending`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending(val: crate::types::ResetPasswordResultPending) -> Self {
        Self::Pending(Box::new(val))
    }

    /// Convenience constructor to create a [`ResetPasswordResult::Declined`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn declined(val: crate::types::ResetPasswordResultDeclined) -> Self {
        Self::Declined(Box::new(val))
    }

}

/// Converts a [`crate::types::ResetPasswordResultPending`] into [`ResetPasswordResult`].
impl From<crate::types::ResetPasswordResultPending> for ResetPasswordResult {
    fn from(val: crate::types::ResetPasswordResultPending) -> Self {
        Self::Pending(Box::new(val))
    }
}

/// Converts a [`crate::types::ResetPasswordResultDeclined`] into [`ResetPasswordResult`].
impl From<crate::types::ResetPasswordResultDeclined> for ResetPasswordResult {
    fn from(val: crate::types::ResetPasswordResultDeclined) -> Self {
        Self::Declined(Box::new(val))
    }
}

/// TDLib `Proxy` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Proxy {
    /// Describes a proxy server
    #[serde(rename(serialize = "proxy", deserialize = "proxy"))]
    Proxy(Box<crate::types::Proxy>),
}

impl Proxy {
    /// Convenience constructor to create a [`Proxy::Proxy`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn proxy(val: crate::types::Proxy) -> Self {
        Self::Proxy(Box::new(val))
    }

}

/// Converts a [`crate::types::Proxy`] into [`Proxy`].
impl From<crate::types::Proxy> for Proxy {
    fn from(val: crate::types::Proxy) -> Self {
        Self::Proxy(Box::new(val))
    }
}

/// Represents the value of an option
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum OptionValue {
    /// Represents a boolean option
    #[serde(rename(serialize = "optionValueBoolean", deserialize = "optionValueBoolean"))]
    Boolean(Box<crate::types::OptionValueBoolean>),
    /// Represents an unknown option or an option which has a default value
    #[serde(rename(serialize = "optionValueEmpty", deserialize = "optionValueEmpty"))]
    Empty,
    /// Represents an integer option
    #[serde(rename(serialize = "optionValueInteger", deserialize = "optionValueInteger"))]
    Integer(Box<crate::types::OptionValueInteger>),
    /// Represents a string option
    #[serde(rename(serialize = "optionValueString", deserialize = "optionValueString"))]
    String(Box<crate::types::OptionValueString>),
}

impl OptionValue {
    /// Convenience constructor to create a [`OptionValue::Boolean`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn boolean(val: crate::types::OptionValueBoolean) -> Self {
        Self::Boolean(Box::new(val))
    }

    /// Convenience constructor to create a [`OptionValue::Integer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn integer(val: crate::types::OptionValueInteger) -> Self {
        Self::Integer(Box::new(val))
    }

    /// Convenience constructor to create a [`OptionValue::String`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn string(val: crate::types::OptionValueString) -> Self {
        Self::String(Box::new(val))
    }

}

/// Converts a [`crate::types::OptionValueBoolean`] into [`OptionValue`].
impl From<crate::types::OptionValueBoolean> for OptionValue {
    fn from(val: crate::types::OptionValueBoolean) -> Self {
        Self::Boolean(Box::new(val))
    }
}

/// Converts a [`crate::types::OptionValueInteger`] into [`OptionValue`].
impl From<crate::types::OptionValueInteger> for OptionValue {
    fn from(val: crate::types::OptionValueInteger) -> Self {
        Self::Integer(Box::new(val))
    }
}

/// Converts a [`crate::types::OptionValueString`] into [`OptionValue`].
impl From<crate::types::OptionValueString> for OptionValue {
    fn from(val: crate::types::OptionValueString) -> Self {
        Self::String(Box::new(val))
    }
}

/// TDLib `JsonObjectMember` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum JsonObjectMember {
    /// Represents one member of a JSON object
    #[serde(rename(serialize = "jsonObjectMember", deserialize = "jsonObjectMember"))]
    JsonObjectMember(Box<crate::types::JsonObjectMember>),
}

impl JsonObjectMember {
    /// Convenience constructor to create a [`JsonObjectMember::JsonObjectMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn json_object_member(val: crate::types::JsonObjectMember) -> Self {
        Self::JsonObjectMember(Box::new(val))
    }

}

/// Converts a [`crate::types::JsonObjectMember`] into [`JsonObjectMember`].
impl From<crate::types::JsonObjectMember> for JsonObjectMember {
    fn from(val: crate::types::JsonObjectMember) -> Self {
        Self::JsonObjectMember(Box::new(val))
    }
}

/// Represents a JSON value
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum JsonValue {
    /// Represents a null JSON value
    #[serde(rename(serialize = "jsonValueNull", deserialize = "jsonValueNull"))]
    Null,
    /// Represents a boolean JSON value
    #[serde(rename(serialize = "jsonValueBoolean", deserialize = "jsonValueBoolean"))]
    Boolean(Box<crate::types::JsonValueBoolean>),
    /// Represents a numeric JSON value
    #[serde(rename(serialize = "jsonValueNumber", deserialize = "jsonValueNumber"))]
    Number(Box<crate::types::JsonValueNumber>),
    /// Represents a string JSON value
    #[serde(rename(serialize = "jsonValueString", deserialize = "jsonValueString"))]
    String(Box<crate::types::JsonValueString>),
    /// Represents a JSON array
    #[serde(rename(serialize = "jsonValueArray", deserialize = "jsonValueArray"))]
    Array(Box<crate::types::JsonValueArray>),
    /// Represents a JSON object
    #[serde(rename(serialize = "jsonValueObject", deserialize = "jsonValueObject"))]
    Object(Box<crate::types::JsonValueObject>),
}

impl JsonValue {
    /// Convenience constructor to create a [`JsonValue::Boolean`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn boolean(val: crate::types::JsonValueBoolean) -> Self {
        Self::Boolean(Box::new(val))
    }

    /// Convenience constructor to create a [`JsonValue::Number`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn number(val: crate::types::JsonValueNumber) -> Self {
        Self::Number(Box::new(val))
    }

    /// Convenience constructor to create a [`JsonValue::String`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn string(val: crate::types::JsonValueString) -> Self {
        Self::String(Box::new(val))
    }

    /// Convenience constructor to create a [`JsonValue::Array`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn array(val: crate::types::JsonValueArray) -> Self {
        Self::Array(Box::new(val))
    }

    /// Convenience constructor to create a [`JsonValue::Object`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn object(val: crate::types::JsonValueObject) -> Self {
        Self::Object(Box::new(val))
    }

}

/// Converts a [`crate::types::JsonValueBoolean`] into [`JsonValue`].
impl From<crate::types::JsonValueBoolean> for JsonValue {
    fn from(val: crate::types::JsonValueBoolean) -> Self {
        Self::Boolean(Box::new(val))
    }
}

/// Converts a [`crate::types::JsonValueNumber`] into [`JsonValue`].
impl From<crate::types::JsonValueNumber> for JsonValue {
    fn from(val: crate::types::JsonValueNumber) -> Self {
        Self::Number(Box::new(val))
    }
}

/// Converts a [`crate::types::JsonValueString`] into [`JsonValue`].
impl From<crate::types::JsonValueString> for JsonValue {
    fn from(val: crate::types::JsonValueString) -> Self {
        Self::String(Box::new(val))
    }
}

/// Converts a [`crate::types::JsonValueArray`] into [`JsonValue`].
impl From<crate::types::JsonValueArray> for JsonValue {
    fn from(val: crate::types::JsonValueArray) -> Self {
        Self::Array(Box::new(val))
    }
}

/// Converts a [`crate::types::JsonValueObject`] into [`JsonValue`].
impl From<crate::types::JsonValueObject> for JsonValue {
    fn from(val: crate::types::JsonValueObject) -> Self {
        Self::Object(Box::new(val))
    }
}

/// TDLib `ReadDatePrivacySettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReadDatePrivacySettings {
    /// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
    #[serde(rename(serialize = "readDatePrivacySettings", deserialize = "readDatePrivacySettings"))]
    ReadDatePrivacySettings(Box<crate::types::ReadDatePrivacySettings>),
}

impl ReadDatePrivacySettings {
    /// Convenience constructor to create a [`ReadDatePrivacySettings::ReadDatePrivacySettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn read_date_privacy_settings(val: crate::types::ReadDatePrivacySettings) -> Self {
        Self::ReadDatePrivacySettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ReadDatePrivacySettings`] into [`ReadDatePrivacySettings`].
impl From<crate::types::ReadDatePrivacySettings> for ReadDatePrivacySettings {
    fn from(val: crate::types::ReadDatePrivacySettings) -> Self {
        Self::ReadDatePrivacySettings(Box::new(val))
    }
}

/// TDLib `AccountTtl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccountTtl {
    /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
    #[serde(rename(serialize = "accountTtl", deserialize = "accountTtl"))]
    AccountTtl(Box<crate::types::AccountTtl>),
}

impl AccountTtl {
    /// Convenience constructor to create a [`AccountTtl::AccountTtl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn account_ttl(val: crate::types::AccountTtl) -> Self {
        Self::AccountTtl(Box::new(val))
    }

}

/// Converts a [`crate::types::AccountTtl`] into [`AccountTtl`].
impl From<crate::types::AccountTtl> for AccountTtl {
    fn from(val: crate::types::AccountTtl) -> Self {
        Self::AccountTtl(Box::new(val))
    }
}

/// Describes type of user session
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SessionType {
    /// A regular session from a device
    #[serde(rename(serialize = "sessionTypeDevice", deserialize = "sessionTypeDevice"))]
    Device(Box<crate::types::SessionTypeDevice>),
    /// A business bot connected to the current user's account
    #[serde(rename(serialize = "sessionTypeConnectedBot", deserialize = "sessionTypeConnectedBot"))]
    ConnectedBot(Box<crate::types::SessionTypeConnectedBot>),
}

impl SessionType {
    /// Convenience constructor to create a [`SessionType::Device`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn device(val: crate::types::SessionTypeDevice) -> Self {
        Self::Device(Box::new(val))
    }

    /// Convenience constructor to create a [`SessionType::ConnectedBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_bot(val: crate::types::SessionTypeConnectedBot) -> Self {
        Self::ConnectedBot(Box::new(val))
    }

}

/// Converts a [`crate::types::SessionTypeDevice`] into [`SessionType`].
impl From<crate::types::SessionTypeDevice> for SessionType {
    fn from(val: crate::types::SessionTypeDevice) -> Self {
        Self::Device(Box::new(val))
    }
}

/// Converts a [`crate::types::SessionTypeConnectedBot`] into [`SessionType`].
impl From<crate::types::SessionTypeConnectedBot> for SessionType {
    fn from(val: crate::types::SessionTypeConnectedBot) -> Self {
        Self::ConnectedBot(Box::new(val))
    }
}

/// Represents the type of device from which session was created
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SessionDeviceType {
    /// The session is running on an Android device
    #[serde(rename(serialize = "sessionDeviceTypeAndroid", deserialize = "sessionDeviceTypeAndroid"))]
    Android,
    /// The session is running on a generic Apple device
    #[serde(rename(serialize = "sessionDeviceTypeApple", deserialize = "sessionDeviceTypeApple"))]
    Apple,
    /// The session is running on the Brave browser
    #[serde(rename(serialize = "sessionDeviceTypeBrave", deserialize = "sessionDeviceTypeBrave"))]
    Brave,
    /// The session is running on the Chrome browser
    #[serde(rename(serialize = "sessionDeviceTypeChrome", deserialize = "sessionDeviceTypeChrome"))]
    Chrome,
    /// The session is running on the Edge browser
    #[serde(rename(serialize = "sessionDeviceTypeEdge", deserialize = "sessionDeviceTypeEdge"))]
    Edge,
    /// The session is running on the Firefox browser
    #[serde(rename(serialize = "sessionDeviceTypeFirefox", deserialize = "sessionDeviceTypeFirefox"))]
    Firefox,
    /// The session is running on an iPad device
    #[serde(rename(serialize = "sessionDeviceTypeIpad", deserialize = "sessionDeviceTypeIpad"))]
    Ipad,
    /// The session is running on an iPhone device
    #[serde(rename(serialize = "sessionDeviceTypeIphone", deserialize = "sessionDeviceTypeIphone"))]
    Iphone,
    /// The session is running on a Linux device
    #[serde(rename(serialize = "sessionDeviceTypeLinux", deserialize = "sessionDeviceTypeLinux"))]
    Linux,
    /// The session is running on a Mac device
    #[serde(rename(serialize = "sessionDeviceTypeMac", deserialize = "sessionDeviceTypeMac"))]
    Mac,
    /// The session is running on the Opera browser
    #[serde(rename(serialize = "sessionDeviceTypeOpera", deserialize = "sessionDeviceTypeOpera"))]
    Opera,
    /// The session is running on the Safari browser
    #[serde(rename(serialize = "sessionDeviceTypeSafari", deserialize = "sessionDeviceTypeSafari"))]
    Safari,
    /// The session is running on an Ubuntu device
    #[serde(rename(serialize = "sessionDeviceTypeUbuntu", deserialize = "sessionDeviceTypeUbuntu"))]
    Ubuntu,
    /// The session is running on an unknown type of device
    #[serde(rename(serialize = "sessionDeviceTypeUnknown", deserialize = "sessionDeviceTypeUnknown"))]
    Unknown,
    /// The session is running on the Vivaldi browser
    #[serde(rename(serialize = "sessionDeviceTypeVivaldi", deserialize = "sessionDeviceTypeVivaldi"))]
    Vivaldi,
    /// The session is running on a Windows device
    #[serde(rename(serialize = "sessionDeviceTypeWindows", deserialize = "sessionDeviceTypeWindows"))]
    Windows,
    /// The session is running on an Xbox console
    #[serde(rename(serialize = "sessionDeviceTypeXbox", deserialize = "sessionDeviceTypeXbox"))]
    Xbox,
}

/// TDLib `Session` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Session {
    /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
    #[serde(rename(serialize = "session", deserialize = "session"))]
    Session(Box<crate::types::Session>),
}

impl Session {
    /// Convenience constructor to create a [`Session::Session`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn session(val: crate::types::Session) -> Self {
        Self::Session(Box::new(val))
    }

}

/// Converts a [`crate::types::Session`] into [`Session`].
impl From<crate::types::Session> for Session {
    fn from(val: crate::types::Session) -> Self {
        Self::Session(Box::new(val))
    }
}

/// TDLib `Sessions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Sessions {
    /// Contains a list of sessions
    #[serde(rename(serialize = "sessions", deserialize = "sessions"))]
    Sessions(Box<crate::types::Sessions>),
}

impl Sessions {
    /// Convenience constructor to create a [`Sessions::Sessions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sessions(val: crate::types::Sessions) -> Self {
        Self::Sessions(Box::new(val))
    }

}

/// Converts a [`crate::types::Sessions`] into [`Sessions`].
impl From<crate::types::Sessions> for Sessions {
    fn from(val: crate::types::Sessions) -> Self {
        Self::Sessions(Box::new(val))
    }
}

/// TDLib `UnconfirmedSession` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UnconfirmedSession {
    /// Contains information about an unconfirmed session
    #[serde(rename(serialize = "unconfirmedSession", deserialize = "unconfirmedSession"))]
    UnconfirmedSession(Box<crate::types::UnconfirmedSession>),
}

impl UnconfirmedSession {
    /// Convenience constructor to create a [`UnconfirmedSession::UnconfirmedSession`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unconfirmed_session(val: crate::types::UnconfirmedSession) -> Self {
        Self::UnconfirmedSession(Box::new(val))
    }

}

/// Converts a [`crate::types::UnconfirmedSession`] into [`UnconfirmedSession`].
impl From<crate::types::UnconfirmedSession> for UnconfirmedSession {
    fn from(val: crate::types::UnconfirmedSession) -> Self {
        Self::UnconfirmedSession(Box::new(val))
    }
}

/// TDLib `ConnectedWebsite` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedWebsite {
    /// Contains information about one website the current user is logged in with Telegram
    #[serde(rename(serialize = "connectedWebsite", deserialize = "connectedWebsite"))]
    ConnectedWebsite(Box<crate::types::ConnectedWebsite>),
}

impl ConnectedWebsite {
    /// Convenience constructor to create a [`ConnectedWebsite::ConnectedWebsite`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_website(val: crate::types::ConnectedWebsite) -> Self {
        Self::ConnectedWebsite(Box::new(val))
    }

}

/// Converts a [`crate::types::ConnectedWebsite`] into [`ConnectedWebsite`].
impl From<crate::types::ConnectedWebsite> for ConnectedWebsite {
    fn from(val: crate::types::ConnectedWebsite) -> Self {
        Self::ConnectedWebsite(Box::new(val))
    }
}

/// TDLib `ConnectedWebsites` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectedWebsites {
    /// Contains a list of websites the current user is logged in with Telegram
    #[serde(rename(serialize = "connectedWebsites", deserialize = "connectedWebsites"))]
    ConnectedWebsites(Box<crate::types::ConnectedWebsites>),
}

impl ConnectedWebsites {
    /// Convenience constructor to create a [`ConnectedWebsites::ConnectedWebsites`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connected_websites(val: crate::types::ConnectedWebsites) -> Self {
        Self::ConnectedWebsites(Box::new(val))
    }

}

/// Converts a [`crate::types::ConnectedWebsites`] into [`ConnectedWebsites`].
impl From<crate::types::ConnectedWebsites> for ConnectedWebsites {
    fn from(val: crate::types::ConnectedWebsites) -> Self {
        Self::ConnectedWebsites(Box::new(val))
    }
}

/// Describes the reason why a chat is reported
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportReason {
    /// The chat contains spam messages
    #[serde(rename(serialize = "reportReasonSpam", deserialize = "reportReasonSpam"))]
    Spam,
    /// The chat promotes violence
    #[serde(rename(serialize = "reportReasonViolence", deserialize = "reportReasonViolence"))]
    Violence,
    /// The chat contains pornographic messages
    #[serde(rename(serialize = "reportReasonPornography", deserialize = "reportReasonPornography"))]
    Pornography,
    /// The chat has child abuse related content
    #[serde(rename(serialize = "reportReasonChildAbuse", deserialize = "reportReasonChildAbuse"))]
    ChildAbuse,
    /// The chat contains copyrighted content
    #[serde(rename(serialize = "reportReasonCopyright", deserialize = "reportReasonCopyright"))]
    Copyright,
    /// The location-based chat is unrelated to its stated location
    #[serde(rename(serialize = "reportReasonUnrelatedLocation", deserialize = "reportReasonUnrelatedLocation"))]
    UnrelatedLocation,
    /// The chat represents a fake account
    #[serde(rename(serialize = "reportReasonFake", deserialize = "reportReasonFake"))]
    Fake,
    /// The chat has illegal drugs related content
    #[serde(rename(serialize = "reportReasonIllegalDrugs", deserialize = "reportReasonIllegalDrugs"))]
    IllegalDrugs,
    /// The chat contains messages with personal details
    #[serde(rename(serialize = "reportReasonPersonalDetails", deserialize = "reportReasonPersonalDetails"))]
    PersonalDetails,
    /// A custom reason provided by the user
    #[serde(rename(serialize = "reportReasonCustom", deserialize = "reportReasonCustom"))]
    Custom,
}

/// Describes a section of the application settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SettingsSection {
    /// The appearance section
    #[serde(rename(serialize = "settingsSectionAppearance", deserialize = "settingsSectionAppearance"))]
    Appearance(Box<crate::types::SettingsSectionAppearance>),
    /// The "Ask a question" section
    #[serde(rename(serialize = "settingsSectionAskQuestion", deserialize = "settingsSectionAskQuestion"))]
    AskQuestion,
    /// The "Telegram Business" section
    #[serde(rename(serialize = "settingsSectionBusiness", deserialize = "settingsSectionBusiness"))]
    Business(Box<crate::types::SettingsSectionBusiness>),
    /// The chat folder settings section
    #[serde(rename(serialize = "settingsSectionChatFolders", deserialize = "settingsSectionChatFolders"))]
    ChatFolders(Box<crate::types::SettingsSectionChatFolders>),
    /// The data and storage settings section
    #[serde(rename(serialize = "settingsSectionDataAndStorage", deserialize = "settingsSectionDataAndStorage"))]
    DataAndStorage(Box<crate::types::SettingsSectionDataAndStorage>),
    /// The Devices section
    #[serde(rename(serialize = "settingsSectionDevices", deserialize = "settingsSectionDevices"))]
    Devices(Box<crate::types::SettingsSectionDevices>),
    /// The profile edit section
    #[serde(rename(serialize = "settingsSectionEditProfile", deserialize = "settingsSectionEditProfile"))]
    EditProfile(Box<crate::types::SettingsSectionEditProfile>),
    /// The FAQ section
    #[serde(rename(serialize = "settingsSectionFaq", deserialize = "settingsSectionFaq"))]
    Faq,
    /// The "Telegram Features" section
    #[serde(rename(serialize = "settingsSectionFeatures", deserialize = "settingsSectionFeatures"))]
    Features,
    /// The in-app browser settings section
    #[serde(rename(serialize = "settingsSectionInAppBrowser", deserialize = "settingsSectionInAppBrowser"))]
    InAppBrowser(Box<crate::types::SettingsSectionInAppBrowser>),
    /// The application language section
    #[serde(rename(serialize = "settingsSectionLanguage", deserialize = "settingsSectionLanguage"))]
    Language(Box<crate::types::SettingsSectionLanguage>),
    /// The Telegram Star balance and transaction section
    #[serde(rename(serialize = "settingsSectionMyStars", deserialize = "settingsSectionMyStars"))]
    MyStars(Box<crate::types::SettingsSectionMyStars>),
    /// The TON Gram balance and transaction section
    #[serde(rename(serialize = "settingsSectionMyGrams", deserialize = "settingsSectionMyGrams"))]
    MyGrams,
    /// The notification settings section
    #[serde(rename(serialize = "settingsSectionNotifications", deserialize = "settingsSectionNotifications"))]
    Notifications(Box<crate::types::SettingsSectionNotifications>),
    /// The power saving settings section
    #[serde(rename(serialize = "settingsSectionPowerSaving", deserialize = "settingsSectionPowerSaving"))]
    PowerSaving(Box<crate::types::SettingsSectionPowerSaving>),
    /// The "Telegram Premium" section
    #[serde(rename(serialize = "settingsSectionPremium", deserialize = "settingsSectionPremium"))]
    Premium,
    /// The privacy and security section
    #[serde(rename(serialize = "settingsSectionPrivacyAndSecurity", deserialize = "settingsSectionPrivacyAndSecurity"))]
    PrivacyAndSecurity(Box<crate::types::SettingsSectionPrivacyAndSecurity>),
    /// The "Privacy Policy" section
    #[serde(rename(serialize = "settingsSectionPrivacyPolicy", deserialize = "settingsSectionPrivacyPolicy"))]
    PrivacyPolicy,
    /// The current user's QR code section
    #[serde(rename(serialize = "settingsSectionQrCode", deserialize = "settingsSectionQrCode"))]
    QrCode(Box<crate::types::SettingsSectionQrCode>),
    /// Search in Settings
    #[serde(rename(serialize = "settingsSectionSearch", deserialize = "settingsSectionSearch"))]
    Search,
    /// The "Send a gift" section
    #[serde(rename(serialize = "settingsSectionSendGift", deserialize = "settingsSectionSendGift"))]
    SendGift(Box<crate::types::SettingsSectionSendGift>),
}

impl SettingsSection {
    /// Convenience constructor to create a [`SettingsSection::Appearance`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn appearance(val: crate::types::SettingsSectionAppearance) -> Self {
        Self::Appearance(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::Business`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business(val: crate::types::SettingsSectionBusiness) -> Self {
        Self::Business(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::ChatFolders`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folders(val: crate::types::SettingsSectionChatFolders) -> Self {
        Self::ChatFolders(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::DataAndStorage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data_and_storage(val: crate::types::SettingsSectionDataAndStorage) -> Self {
        Self::DataAndStorage(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::Devices`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn devices(val: crate::types::SettingsSectionDevices) -> Self {
        Self::Devices(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::EditProfile`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn edit_profile(val: crate::types::SettingsSectionEditProfile) -> Self {
        Self::EditProfile(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::InAppBrowser`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn in_app_browser(val: crate::types::SettingsSectionInAppBrowser) -> Self {
        Self::InAppBrowser(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::Language`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language(val: crate::types::SettingsSectionLanguage) -> Self {
        Self::Language(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::MyStars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn my_stars(val: crate::types::SettingsSectionMyStars) -> Self {
        Self::MyStars(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::Notifications`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notifications(val: crate::types::SettingsSectionNotifications) -> Self {
        Self::Notifications(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::PowerSaving`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn power_saving(val: crate::types::SettingsSectionPowerSaving) -> Self {
        Self::PowerSaving(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::PrivacyAndSecurity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn privacy_and_security(val: crate::types::SettingsSectionPrivacyAndSecurity) -> Self {
        Self::PrivacyAndSecurity(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::QrCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn qr_code(val: crate::types::SettingsSectionQrCode) -> Self {
        Self::QrCode(Box::new(val))
    }

    /// Convenience constructor to create a [`SettingsSection::SendGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn send_gift(val: crate::types::SettingsSectionSendGift) -> Self {
        Self::SendGift(Box::new(val))
    }

}

/// Converts a [`crate::types::SettingsSectionAppearance`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionAppearance> for SettingsSection {
    fn from(val: crate::types::SettingsSectionAppearance) -> Self {
        Self::Appearance(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionBusiness`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionBusiness> for SettingsSection {
    fn from(val: crate::types::SettingsSectionBusiness) -> Self {
        Self::Business(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionChatFolders`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionChatFolders> for SettingsSection {
    fn from(val: crate::types::SettingsSectionChatFolders) -> Self {
        Self::ChatFolders(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionDataAndStorage`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionDataAndStorage> for SettingsSection {
    fn from(val: crate::types::SettingsSectionDataAndStorage) -> Self {
        Self::DataAndStorage(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionDevices`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionDevices> for SettingsSection {
    fn from(val: crate::types::SettingsSectionDevices) -> Self {
        Self::Devices(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionEditProfile`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionEditProfile> for SettingsSection {
    fn from(val: crate::types::SettingsSectionEditProfile) -> Self {
        Self::EditProfile(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionInAppBrowser`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionInAppBrowser> for SettingsSection {
    fn from(val: crate::types::SettingsSectionInAppBrowser) -> Self {
        Self::InAppBrowser(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionLanguage`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionLanguage> for SettingsSection {
    fn from(val: crate::types::SettingsSectionLanguage) -> Self {
        Self::Language(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionMyStars`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionMyStars> for SettingsSection {
    fn from(val: crate::types::SettingsSectionMyStars) -> Self {
        Self::MyStars(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionNotifications`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionNotifications> for SettingsSection {
    fn from(val: crate::types::SettingsSectionNotifications) -> Self {
        Self::Notifications(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionPowerSaving`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionPowerSaving> for SettingsSection {
    fn from(val: crate::types::SettingsSectionPowerSaving) -> Self {
        Self::PowerSaving(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionPrivacyAndSecurity`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionPrivacyAndSecurity> for SettingsSection {
    fn from(val: crate::types::SettingsSectionPrivacyAndSecurity) -> Self {
        Self::PrivacyAndSecurity(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionQrCode`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionQrCode> for SettingsSection {
    fn from(val: crate::types::SettingsSectionQrCode) -> Self {
        Self::QrCode(Box::new(val))
    }
}

/// Converts a [`crate::types::SettingsSectionSendGift`] into [`SettingsSection`].
impl From<crate::types::SettingsSectionSendGift> for SettingsSection {
    fn from(val: crate::types::SettingsSectionSendGift) -> Self {
        Self::SendGift(Box::new(val))
    }
}

/// Describes an internal https:t.me or tg: link, which must be processed by the application in a special way
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InternalLinkType {
    /// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat.
    /// Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot.
    /// If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu.
    /// If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
    #[serde(rename(serialize = "internalLinkTypeAttachmentMenuBot", deserialize = "internalLinkTypeAttachmentMenuBot"))]
    AttachmentMenuBot(Box<crate::types::InternalLinkTypeAttachmentMenuBot>),
    /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
    #[serde(rename(serialize = "internalLinkTypeAuthenticationCode", deserialize = "internalLinkTypeAuthenticationCode"))]
    AuthenticationCode(Box<crate::types::InternalLinkTypeAuthenticationCode>),
    /// The link is a link to a background. Call searchBackground with the given background name to process the link.
    /// If background is found and the user wants to apply it, then call setDefaultBackground
    #[serde(rename(serialize = "internalLinkTypeBackground", deserialize = "internalLinkTypeBackground"))]
    Background(Box<crate::types::InternalLinkTypeBackground>),
    /// The link is a link to a Telegram bot, which is expected to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot,
    /// ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
    /// check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
    #[serde(rename(serialize = "internalLinkTypeBotAddToChannel", deserialize = "internalLinkTypeBotAddToChannel"))]
    BotAddToChannel(Box<crate::types::InternalLinkTypeBotAddToChannel>),
    /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot,
    /// and then call sendBotStartMessage with the given start parameter after the button is pressed
    #[serde(rename(serialize = "internalLinkTypeBotStart", deserialize = "internalLinkTypeBotStart"))]
    BotStart(Box<crate::types::InternalLinkTypeBotStart>),
    /// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups,
    /// ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup.
    /// If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
    /// check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user,
    /// and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat.
    /// Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
    #[serde(rename(serialize = "internalLinkTypeBotStartInGroup", deserialize = "internalLinkTypeBotStartInGroup"))]
    BotStartInGroup(Box<crate::types::InternalLinkTypeBotStartInGroup>),
    /// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link,
    /// then open received private chat and replace chat draft with the provided text
    #[serde(rename(serialize = "internalLinkTypeBusinessChat", deserialize = "internalLinkTypeBusinessChat"))]
    BusinessChat(Box<crate::types::InternalLinkTypeBusinessChat>),
    /// The link is a link to the Call tab or page
    #[serde(rename(serialize = "internalLinkTypeCallsPage", deserialize = "internalLinkTypeCallsPage"))]
    CallsPage(Box<crate::types::InternalLinkTypeCallsPage>),
    /// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
    #[serde(rename(serialize = "internalLinkTypeChatAffiliateProgram", deserialize = "internalLinkTypeChatAffiliateProgram"))]
    ChatAffiliateProgram(Box<crate::types::InternalLinkTypeChatAffiliateProgram>),
    /// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link.
    /// If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted.
    /// If the user wants to boost the chat and the chat can be boosted, then call boostChat
    #[serde(rename(serialize = "internalLinkTypeChatBoost", deserialize = "internalLinkTypeChatBoost"))]
    ChatBoost(Box<crate::types::InternalLinkTypeChatBoost>),
    /// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link.
    /// If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
    #[serde(rename(serialize = "internalLinkTypeChatFolderInvite", deserialize = "internalLinkTypeChatFolderInvite"))]
    ChatFolderInvite(Box<crate::types::InternalLinkTypeChatFolderInvite>),
    /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link.
    /// If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
    #[serde(rename(serialize = "internalLinkTypeChatInvite", deserialize = "internalLinkTypeChatInvite"))]
    ChatInvite(Box<crate::types::InternalLinkTypeChatInvite>),
    /// The link is a link that allows to select some chats
    #[serde(rename(serialize = "internalLinkTypeChatSelection", deserialize = "internalLinkTypeChatSelection"))]
    ChatSelection,
    /// The link is a link to the Contacts tab or page
    #[serde(rename(serialize = "internalLinkTypeContactsPage", deserialize = "internalLinkTypeContactsPage"))]
    ContactsPage(Box<crate::types::InternalLinkTypeContactsPage>),
    /// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link.
    /// If the chat is found and is channel, open the direct messages chat of the channel
    #[serde(rename(serialize = "internalLinkTypeDirectMessagesChat", deserialize = "internalLinkTypeDirectMessagesChat"))]
    DirectMessagesChat(Box<crate::types::InternalLinkTypeDirectMessagesChat>),
    /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot,
    /// ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
    #[serde(rename(serialize = "internalLinkTypeGame", deserialize = "internalLinkTypeGame"))]
    Game(Box<crate::types::InternalLinkTypeGame>),
    /// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
    #[serde(rename(serialize = "internalLinkTypeGiftAuction", deserialize = "internalLinkTypeGiftAuction"))]
    GiftAuction(Box<crate::types::InternalLinkTypeGiftAuction>),
    /// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier
    /// and the given collection identifier, then show the collection if received
    #[serde(rename(serialize = "internalLinkTypeGiftCollection", deserialize = "internalLinkTypeGiftCollection"))]
    GiftCollection(Box<crate::types::InternalLinkTypeGiftCollection>),
    /// The link is a link to a group call that isn't bound to a chat. Use getGroupCallParticipants to get the list of group call participants and show them on the join group call screen.
    /// Call joinGroupCall with the given invite_link to join the call
    #[serde(rename(serialize = "internalLinkTypeGroupCall", deserialize = "internalLinkTypeGroupCall"))]
    GroupCall(Box<crate::types::InternalLinkTypeGroupCall>),
    /// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link.
    /// If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
    #[serde(rename(serialize = "internalLinkTypeInstantView", deserialize = "internalLinkTypeInstantView"))]
    InstantView(Box<crate::types::InternalLinkTypeInstantView>),
    /// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
    #[serde(rename(serialize = "internalLinkTypeInvoice", deserialize = "internalLinkTypeInvoice"))]
    Invoice(Box<crate::types::InternalLinkTypeInvoice>),
    /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link.
    /// If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
    #[serde(rename(serialize = "internalLinkTypeLanguagePack", deserialize = "internalLinkTypeLanguagePack"))]
    LanguagePack(Box<crate::types::InternalLinkTypeLanguagePack>),
    /// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat,
    /// then find a live story among active stories of the chat, and then joinLiveStory to join the live story
    #[serde(rename(serialize = "internalLinkTypeLiveStory", deserialize = "internalLinkTypeLiveStory"))]
    LiveStory(Box<crate::types::InternalLinkTypeLiveStory>),
    /// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App.
    /// If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu,
    /// show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu,
    /// then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
    #[serde(rename(serialize = "internalLinkTypeMainWebApp", deserialize = "internalLinkTypeMainWebApp"))]
    MainWebApp(Box<crate::types::InternalLinkTypeMainWebApp>),
    /// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link,
    /// and then open received forum topic or chat and show the message there
    #[serde(rename(serialize = "internalLinkTypeMessage", deserialize = "internalLinkTypeMessage"))]
    Message(Box<crate::types::InternalLinkTypeMessage>),
    /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
    #[serde(rename(serialize = "internalLinkTypeMessageDraft", deserialize = "internalLinkTypeMessageDraft"))]
    MessageDraft(Box<crate::types::InternalLinkTypeMessageDraft>),
    /// The link is a link to the My Profile application page
    #[serde(rename(serialize = "internalLinkTypeMyProfilePage", deserialize = "internalLinkTypeMyProfilePage"))]
    MyProfilePage(Box<crate::types::InternalLinkTypeMyProfilePage>),
    /// The link is a link to the screen for creating a new channel chat
    #[serde(rename(serialize = "internalLinkTypeNewChannelChat", deserialize = "internalLinkTypeNewChannelChat"))]
    NewChannelChat,
    /// The link is a link to the screen for creating a new group chat
    #[serde(rename(serialize = "internalLinkTypeNewGroupChat", deserialize = "internalLinkTypeNewGroupChat"))]
    NewGroupChat,
    /// The link is a link to the screen for creating a new private chat with a contact
    #[serde(rename(serialize = "internalLinkTypeNewPrivateChat", deserialize = "internalLinkTypeNewPrivateChat"))]
    NewPrivateChat,
    /// The link is a link to open the story posting interface
    #[serde(rename(serialize = "internalLinkTypeNewStory", deserialize = "internalLinkTypeNewStory"))]
    NewStory(Box<crate::types::InternalLinkTypeNewStory>),
    /// The link is an OAuth link. Call getOauthLinkInfo with the given URL to process the link if the link was received from outside of the application; otherwise, ignore it.
    /// After getOauthLinkInfo, show the user confirmation dialog and process it with checkOauthRequestMatchCode, acceptOauthRequest or declineOauthRequest
    #[serde(rename(serialize = "internalLinkTypeOauth", deserialize = "internalLinkTypeOauth"))]
    Oauth(Box<crate::types::InternalLinkTypeOauth>),
    /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
    #[serde(rename(serialize = "internalLinkTypePassportDataRequest", deserialize = "internalLinkTypePassportDataRequest"))]
    PassportDataRequest(Box<crate::types::InternalLinkTypePassportDataRequest>),
    /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link.
    /// If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
    #[serde(rename(serialize = "internalLinkTypePhoneNumberConfirmation", deserialize = "internalLinkTypePhoneNumberConfirmation"))]
    PhoneNumberConfirmation(Box<crate::types::InternalLinkTypePhoneNumberConfirmation>),
    /// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
    #[serde(rename(serialize = "internalLinkTypePremiumFeaturesPage", deserialize = "internalLinkTypePremiumFeaturesPage"))]
    PremiumFeaturesPage(Box<crate::types::InternalLinkTypePremiumFeaturesPage>),
    /// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link.
    /// If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
    #[serde(rename(serialize = "internalLinkTypePremiumGiftCode", deserialize = "internalLinkTypePremiumGiftCode"))]
    PremiumGiftCode(Box<crate::types::InternalLinkTypePremiumGiftCode>),
    /// The link is a link to the screen for gifting Telegram Premium subscriptions to friends via inputInvoiceTelegram with telegramPaymentPurposePremiumGift payments or in-store purchases
    #[serde(rename(serialize = "internalLinkTypePremiumGiftPurchase", deserialize = "internalLinkTypePremiumGiftPurchase"))]
    PremiumGiftPurchase(Box<crate::types::InternalLinkTypePremiumGiftPurchase>),
    /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
    #[serde(rename(serialize = "internalLinkTypeProxy", deserialize = "internalLinkTypeProxy"))]
    Proxy(Box<crate::types::InternalLinkTypeProxy>),
    /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link.
    /// If the chat is found, open its profile information screen or the chat itself.
    /// If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
    #[serde(rename(serialize = "internalLinkTypePublicChat", deserialize = "internalLinkTypePublicChat"))]
    PublicChat(Box<crate::types::InternalLinkTypePublicChat>),
    /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to
    /// "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
    #[serde(rename(serialize = "internalLinkTypeQrCodeAuthentication", deserialize = "internalLinkTypeQrCodeAuthentication"))]
    QrCodeAuthentication,
    /// The link is a link to a dialog for creating of a managed bot. Call searchPublicChat with the given manager bot username.
    /// If the chat is found, the chat is a chat with a bot and the bot has can_manage_bots == true, then show bot creation confirmation dialog
    /// with the given suggested_bot_username and suggested_bot_name. If user agrees, call createBot with via_link == true to create the bot
    #[serde(rename(serialize = "internalLinkTypeRequestManagedBot", deserialize = "internalLinkTypeRequestManagedBot"))]
    RequestManagedBot(Box<crate::types::InternalLinkTypeRequestManagedBot>),
    /// The link forces restore of App Store purchases when opened. For official iOS application only
    #[serde(rename(serialize = "internalLinkTypeRestorePurchases", deserialize = "internalLinkTypeRestorePurchases"))]
    RestorePurchases,
    /// The link is a link to the Saved Messages chat. Call createPrivateChat with getOption("my_id") and open the chat
    #[serde(rename(serialize = "internalLinkTypeSavedMessages", deserialize = "internalLinkTypeSavedMessages"))]
    SavedMessages,
    /// The link is a link to the global chat and messages search field
    #[serde(rename(serialize = "internalLinkTypeSearch", deserialize = "internalLinkTypeSearch"))]
    Search,
    /// The link is a link to application settings
    #[serde(rename(serialize = "internalLinkTypeSettings", deserialize = "internalLinkTypeSettings"))]
    Settings(Box<crate::types::InternalLinkTypeSettings>),
    /// The link is a link to the Telegram Star purchase section of the application
    #[serde(rename(serialize = "internalLinkTypeStarPurchase", deserialize = "internalLinkTypeStarPurchase"))]
    StarPurchase(Box<crate::types::InternalLinkTypeStarPurchase>),
    /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set.
    /// If the sticker set is found and the user wants to add it, then call changeStickerSet
    #[serde(rename(serialize = "internalLinkTypeStickerSet", deserialize = "internalLinkTypeStickerSet"))]
    StickerSet(Box<crate::types::InternalLinkTypeStickerSet>),
    /// The link is a link to a story. Call searchPublicChat with the given poster username, then call getStory with the received chat identifier and the given story identifier, then show the story if received
    #[serde(rename(serialize = "internalLinkTypeStory", deserialize = "internalLinkTypeStory"))]
    Story(Box<crate::types::InternalLinkTypeStory>),
    /// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier
    /// and the given story album identifier, then show the story album if received
    #[serde(rename(serialize = "internalLinkTypeStoryAlbum", deserialize = "internalLinkTypeStoryAlbum"))]
    StoryAlbum(Box<crate::types::InternalLinkTypeStoryAlbum>),
    /// The link is a link to a text composition style. Call searchTextCompositionStyle with the given style name to get information about the style.
    /// If the style is found and the user wants to add it, then call addTextCompositionStyle
    #[serde(rename(serialize = "internalLinkTypeTextCompositionStyle", deserialize = "internalLinkTypeTextCompositionStyle"))]
    TextCompositionStyle(Box<crate::types::InternalLinkTypeTextCompositionStyle>),
    /// The link is a link to a cloud theme. TDLib has no theme support yet
    #[serde(rename(serialize = "internalLinkTypeTheme", deserialize = "internalLinkTypeTheme"))]
    Theme(Box<crate::types::InternalLinkTypeTheme>),
    /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
    #[serde(rename(serialize = "internalLinkTypeUnknownDeepLink", deserialize = "internalLinkTypeUnknownDeepLink"))]
    UnknownDeepLink(Box<crate::types::InternalLinkTypeUnknownDeepLink>),
    /// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
    #[serde(rename(serialize = "internalLinkTypeUpgradedGift", deserialize = "internalLinkTypeUpgradedGift"))]
    UpgradedGift(Box<crate::types::InternalLinkTypeUpgradedGift>),
    /// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link.
    /// If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
    #[serde(rename(serialize = "internalLinkTypeUserPhoneNumber", deserialize = "internalLinkTypeUserPhoneNumber"))]
    UserPhoneNumber(Box<crate::types::InternalLinkTypeUserPhoneNumber>),
    /// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link.
    /// If the user is found, then call createPrivateChat and open the chat
    #[serde(rename(serialize = "internalLinkTypeUserToken", deserialize = "internalLinkTypeUserToken"))]
    UserToken(Box<crate::types::InternalLinkTypeUserToken>),
    /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinVideoChat with the given invite hash to process the link
    #[serde(rename(serialize = "internalLinkTypeVideoChat", deserialize = "internalLinkTypeVideoChat"))]
    VideoChat(Box<crate::types::InternalLinkTypeVideoChat>),
    /// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message.
    /// Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed.
    /// If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog
    /// and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// Then, call getWebAppLinkUrl and open the returned URL as a Web App
    #[serde(rename(serialize = "internalLinkTypeWebApp", deserialize = "internalLinkTypeWebApp"))]
    WebApp(Box<crate::types::InternalLinkTypeWebApp>),
}

impl InternalLinkType {
    /// Convenience constructor to create a [`InternalLinkType::AttachmentMenuBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn attachment_menu_bot(val: crate::types::InternalLinkTypeAttachmentMenuBot) -> Self {
        Self::AttachmentMenuBot(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::AuthenticationCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn authentication_code(val: crate::types::InternalLinkTypeAuthenticationCode) -> Self {
        Self::AuthenticationCode(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Background`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn background(val: crate::types::InternalLinkTypeBackground) -> Self {
        Self::Background(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::BotAddToChannel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_add_to_channel(val: crate::types::InternalLinkTypeBotAddToChannel) -> Self {
        Self::BotAddToChannel(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::BotStart`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_start(val: crate::types::InternalLinkTypeBotStart) -> Self {
        Self::BotStart(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::BotStartInGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_start_in_group(val: crate::types::InternalLinkTypeBotStartInGroup) -> Self {
        Self::BotStartInGroup(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::BusinessChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_chat(val: crate::types::InternalLinkTypeBusinessChat) -> Self {
        Self::BusinessChat(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::CallsPage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn calls_page(val: crate::types::InternalLinkTypeCallsPage) -> Self {
        Self::CallsPage(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::ChatAffiliateProgram`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_affiliate_program(val: crate::types::InternalLinkTypeChatAffiliateProgram) -> Self {
        Self::ChatAffiliateProgram(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::ChatBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost(val: crate::types::InternalLinkTypeChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::ChatFolderInvite`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folder_invite(val: crate::types::InternalLinkTypeChatFolderInvite) -> Self {
        Self::ChatFolderInvite(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::ChatInvite`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite(val: crate::types::InternalLinkTypeChatInvite) -> Self {
        Self::ChatInvite(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::ContactsPage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contacts_page(val: crate::types::InternalLinkTypeContactsPage) -> Self {
        Self::ContactsPage(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::DirectMessagesChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn direct_messages_chat(val: crate::types::InternalLinkTypeDirectMessagesChat) -> Self {
        Self::DirectMessagesChat(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::InternalLinkTypeGame) -> Self {
        Self::Game(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::GiftAuction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction(val: crate::types::InternalLinkTypeGiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::GiftCollection`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_collection(val: crate::types::InternalLinkTypeGiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::GroupCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call(val: crate::types::InternalLinkTypeGroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::InstantView`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn instant_view(val: crate::types::InternalLinkTypeInstantView) -> Self {
        Self::InstantView(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Invoice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn invoice(val: crate::types::InternalLinkTypeInvoice) -> Self {
        Self::Invoice(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::LanguagePack`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language_pack(val: crate::types::InternalLinkTypeLanguagePack) -> Self {
        Self::LanguagePack(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::LiveStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_story(val: crate::types::InternalLinkTypeLiveStory) -> Self {
        Self::LiveStory(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::MainWebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn main_web_app(val: crate::types::InternalLinkTypeMainWebApp) -> Self {
        Self::MainWebApp(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::InternalLinkTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::MessageDraft`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_draft(val: crate::types::InternalLinkTypeMessageDraft) -> Self {
        Self::MessageDraft(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::MyProfilePage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn my_profile_page(val: crate::types::InternalLinkTypeMyProfilePage) -> Self {
        Self::MyProfilePage(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::NewStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_story(val: crate::types::InternalLinkTypeNewStory) -> Self {
        Self::NewStory(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Oauth`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn oauth(val: crate::types::InternalLinkTypeOauth) -> Self {
        Self::Oauth(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PassportDataRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_data_request(val: crate::types::InternalLinkTypePassportDataRequest) -> Self {
        Self::PassportDataRequest(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PhoneNumberConfirmation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number_confirmation(val: crate::types::InternalLinkTypePhoneNumberConfirmation) -> Self {
        Self::PhoneNumberConfirmation(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PremiumFeaturesPage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_features_page(val: crate::types::InternalLinkTypePremiumFeaturesPage) -> Self {
        Self::PremiumFeaturesPage(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PremiumGiftCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_code(val: crate::types::InternalLinkTypePremiumGiftCode) -> Self {
        Self::PremiumGiftCode(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PremiumGiftPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_purchase(val: crate::types::InternalLinkTypePremiumGiftPurchase) -> Self {
        Self::PremiumGiftPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Proxy`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn proxy(val: crate::types::InternalLinkTypeProxy) -> Self {
        Self::Proxy(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::PublicChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn public_chat(val: crate::types::InternalLinkTypePublicChat) -> Self {
        Self::PublicChat(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::RequestManagedBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn request_managed_bot(val: crate::types::InternalLinkTypeRequestManagedBot) -> Self {
        Self::RequestManagedBot(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Settings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn settings(val: crate::types::InternalLinkTypeSettings) -> Self {
        Self::Settings(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::StarPurchase`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_purchase(val: crate::types::InternalLinkTypeStarPurchase) -> Self {
        Self::StarPurchase(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::StickerSet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set(val: crate::types::InternalLinkTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::InternalLinkTypeStory) -> Self {
        Self::Story(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::StoryAlbum`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_album(val: crate::types::InternalLinkTypeStoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::TextCompositionStyle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_composition_style(val: crate::types::InternalLinkTypeTextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::Theme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn theme(val: crate::types::InternalLinkTypeTheme) -> Self {
        Self::Theme(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::UnknownDeepLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unknown_deep_link(val: crate::types::InternalLinkTypeUnknownDeepLink) -> Self {
        Self::UnknownDeepLink(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::InternalLinkTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::UserPhoneNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_phone_number(val: crate::types::InternalLinkTypeUserPhoneNumber) -> Self {
        Self::UserPhoneNumber(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::UserToken`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_token(val: crate::types::InternalLinkTypeUserToken) -> Self {
        Self::UserToken(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::VideoChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_chat(val: crate::types::InternalLinkTypeVideoChat) -> Self {
        Self::VideoChat(Box::new(val))
    }

    /// Convenience constructor to create a [`InternalLinkType::WebApp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app(val: crate::types::InternalLinkTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }

}

/// Converts a [`crate::types::InternalLinkTypeAttachmentMenuBot`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeAttachmentMenuBot> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeAttachmentMenuBot) -> Self {
        Self::AttachmentMenuBot(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeAuthenticationCode`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeAuthenticationCode> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeAuthenticationCode) -> Self {
        Self::AuthenticationCode(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeBackground`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeBackground> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeBackground) -> Self {
        Self::Background(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeBotAddToChannel`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeBotAddToChannel> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeBotAddToChannel) -> Self {
        Self::BotAddToChannel(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeBotStart`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeBotStart> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeBotStart) -> Self {
        Self::BotStart(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeBotStartInGroup`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeBotStartInGroup> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeBotStartInGroup) -> Self {
        Self::BotStartInGroup(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeBusinessChat`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeBusinessChat> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeBusinessChat) -> Self {
        Self::BusinessChat(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeCallsPage`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeCallsPage> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeCallsPage) -> Self {
        Self::CallsPage(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeChatAffiliateProgram`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeChatAffiliateProgram> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeChatAffiliateProgram) -> Self {
        Self::ChatAffiliateProgram(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeChatBoost`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeChatBoost> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeChatFolderInvite`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeChatFolderInvite> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeChatFolderInvite) -> Self {
        Self::ChatFolderInvite(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeChatInvite`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeChatInvite> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeChatInvite) -> Self {
        Self::ChatInvite(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeContactsPage`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeContactsPage> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeContactsPage) -> Self {
        Self::ContactsPage(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeDirectMessagesChat`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeDirectMessagesChat> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeDirectMessagesChat) -> Self {
        Self::DirectMessagesChat(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeGame`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeGame> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeGame) -> Self {
        Self::Game(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeGiftAuction`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeGiftAuction> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeGiftAuction) -> Self {
        Self::GiftAuction(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeGiftCollection`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeGiftCollection> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeGiftCollection) -> Self {
        Self::GiftCollection(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeGroupCall`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeGroupCall> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeGroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeInstantView`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeInstantView> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeInstantView) -> Self {
        Self::InstantView(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeInvoice`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeInvoice> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeInvoice) -> Self {
        Self::Invoice(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeLanguagePack`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeLanguagePack> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeLanguagePack) -> Self {
        Self::LanguagePack(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeLiveStory`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeLiveStory> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeLiveStory) -> Self {
        Self::LiveStory(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeMainWebApp`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeMainWebApp> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeMainWebApp) -> Self {
        Self::MainWebApp(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeMessage`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeMessage> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeMessageDraft`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeMessageDraft> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeMessageDraft) -> Self {
        Self::MessageDraft(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeMyProfilePage`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeMyProfilePage> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeMyProfilePage) -> Self {
        Self::MyProfilePage(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeNewStory`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeNewStory> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeNewStory) -> Self {
        Self::NewStory(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeOauth`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeOauth> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeOauth) -> Self {
        Self::Oauth(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePassportDataRequest`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePassportDataRequest> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePassportDataRequest) -> Self {
        Self::PassportDataRequest(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePhoneNumberConfirmation`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePhoneNumberConfirmation> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePhoneNumberConfirmation) -> Self {
        Self::PhoneNumberConfirmation(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePremiumFeaturesPage`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePremiumFeaturesPage> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePremiumFeaturesPage) -> Self {
        Self::PremiumFeaturesPage(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePremiumGiftCode`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePremiumGiftCode> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePremiumGiftCode) -> Self {
        Self::PremiumGiftCode(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePremiumGiftPurchase`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePremiumGiftPurchase> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePremiumGiftPurchase) -> Self {
        Self::PremiumGiftPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeProxy`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeProxy> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeProxy) -> Self {
        Self::Proxy(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypePublicChat`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypePublicChat> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypePublicChat) -> Self {
        Self::PublicChat(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeRequestManagedBot`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeRequestManagedBot> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeRequestManagedBot) -> Self {
        Self::RequestManagedBot(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeSettings`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeSettings> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeSettings) -> Self {
        Self::Settings(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeStarPurchase`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeStarPurchase> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeStarPurchase) -> Self {
        Self::StarPurchase(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeStickerSet`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeStickerSet> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeStory`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeStory> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeStoryAlbum`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeStoryAlbum> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeStoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeTextCompositionStyle`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeTextCompositionStyle> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeTextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeTheme`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeTheme> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeTheme) -> Self {
        Self::Theme(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeUnknownDeepLink`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeUnknownDeepLink> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeUnknownDeepLink) -> Self {
        Self::UnknownDeepLink(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeUpgradedGift`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeUpgradedGift> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeUserPhoneNumber`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeUserPhoneNumber> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeUserPhoneNumber) -> Self {
        Self::UserPhoneNumber(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeUserToken`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeUserToken> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeUserToken) -> Self {
        Self::UserToken(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeVideoChat`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeVideoChat> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeVideoChat) -> Self {
        Self::VideoChat(Box::new(val))
    }
}

/// Converts a [`crate::types::InternalLinkTypeWebApp`] into [`InternalLinkType`].
impl From<crate::types::InternalLinkTypeWebApp> for InternalLinkType {
    fn from(val: crate::types::InternalLinkTypeWebApp) -> Self {
        Self::WebApp(Box::new(val))
    }
}

/// Describes type of block list
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BlockList {
    /// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
    #[serde(rename(serialize = "blockListMain", deserialize = "blockListMain"))]
    Main,
    /// The block list that disallows viewing of stories of the current user
    #[serde(rename(serialize = "blockListStories", deserialize = "blockListStories"))]
    Stories,
}

/// TDLib `StorageStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatistics {
    /// Contains the exact storage usage statistics split by chats and file type
    #[serde(rename(serialize = "storageStatistics", deserialize = "storageStatistics"))]
    StorageStatistics(Box<crate::types::StorageStatistics>),
}

impl StorageStatistics {
    /// Convenience constructor to create a [`StorageStatistics::StorageStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn storage_statistics(val: crate::types::StorageStatistics) -> Self {
        Self::StorageStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::StorageStatistics`] into [`StorageStatistics`].
impl From<crate::types::StorageStatistics> for StorageStatistics {
    fn from(val: crate::types::StorageStatistics) -> Self {
        Self::StorageStatistics(Box::new(val))
    }
}

/// TDLib `StorageStatisticsFast` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StorageStatisticsFast {
    /// Contains approximate storage usage statistics, excluding files of unknown file type
    #[serde(rename(serialize = "storageStatisticsFast", deserialize = "storageStatisticsFast"))]
    StorageStatisticsFast(Box<crate::types::StorageStatisticsFast>),
}

impl StorageStatisticsFast {
    /// Convenience constructor to create a [`StorageStatisticsFast::StorageStatisticsFast`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn storage_statistics_fast(val: crate::types::StorageStatisticsFast) -> Self {
        Self::StorageStatisticsFast(Box::new(val))
    }

}

/// Converts a [`crate::types::StorageStatisticsFast`] into [`StorageStatisticsFast`].
impl From<crate::types::StorageStatisticsFast> for StorageStatisticsFast {
    fn from(val: crate::types::StorageStatisticsFast) -> Self {
        Self::StorageStatisticsFast(Box::new(val))
    }
}

/// TDLib `DatabaseStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DatabaseStatistics {
    /// Contains database statistics
    #[serde(rename(serialize = "databaseStatistics", deserialize = "databaseStatistics"))]
    DatabaseStatistics(Box<crate::types::DatabaseStatistics>),
}

impl DatabaseStatistics {
    /// Convenience constructor to create a [`DatabaseStatistics::DatabaseStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn database_statistics(val: crate::types::DatabaseStatistics) -> Self {
        Self::DatabaseStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::DatabaseStatistics`] into [`DatabaseStatistics`].
impl From<crate::types::DatabaseStatistics> for DatabaseStatistics {
    fn from(val: crate::types::DatabaseStatistics) -> Self {
        Self::DatabaseStatistics(Box::new(val))
    }
}

/// Represents the type of network
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkType {
    /// The network is not available
    #[serde(rename(serialize = "networkTypeNone", deserialize = "networkTypeNone"))]
    None,
    /// A mobile network
    #[serde(rename(serialize = "networkTypeMobile", deserialize = "networkTypeMobile"))]
    Mobile,
    /// A mobile roaming network
    #[serde(rename(serialize = "networkTypeMobileRoaming", deserialize = "networkTypeMobileRoaming"))]
    MobileRoaming,
    /// A Wi-Fi network
    #[serde(rename(serialize = "networkTypeWiFi", deserialize = "networkTypeWiFi"))]
    WiFi,
    /// A different network type (e.g., Ethernet network)
    #[serde(rename(serialize = "networkTypeOther", deserialize = "networkTypeOther"))]
    Other,
}

/// Contains statistics about network usage
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkStatisticsEntry {
    /// Contains information about the total amount of data that was used to send and receive files
    #[serde(rename(serialize = "networkStatisticsEntryFile", deserialize = "networkStatisticsEntryFile"))]
    File(Box<crate::types::NetworkStatisticsEntryFile>),
    /// Contains information about the total amount of data that was used for calls
    #[serde(rename(serialize = "networkStatisticsEntryCall", deserialize = "networkStatisticsEntryCall"))]
    Call(Box<crate::types::NetworkStatisticsEntryCall>),
}

impl NetworkStatisticsEntry {
    /// Convenience constructor to create a [`NetworkStatisticsEntry::File`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file(val: crate::types::NetworkStatisticsEntryFile) -> Self {
        Self::File(Box::new(val))
    }

    /// Convenience constructor to create a [`NetworkStatisticsEntry::Call`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call(val: crate::types::NetworkStatisticsEntryCall) -> Self {
        Self::Call(Box::new(val))
    }

}

/// Converts a [`crate::types::NetworkStatisticsEntryFile`] into [`NetworkStatisticsEntry`].
impl From<crate::types::NetworkStatisticsEntryFile> for NetworkStatisticsEntry {
    fn from(val: crate::types::NetworkStatisticsEntryFile) -> Self {
        Self::File(Box::new(val))
    }
}

/// Converts a [`crate::types::NetworkStatisticsEntryCall`] into [`NetworkStatisticsEntry`].
impl From<crate::types::NetworkStatisticsEntryCall> for NetworkStatisticsEntry {
    fn from(val: crate::types::NetworkStatisticsEntryCall) -> Self {
        Self::Call(Box::new(val))
    }
}

/// TDLib `NetworkStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NetworkStatistics {
    /// A full list of available network statistic entries
    #[serde(rename(serialize = "networkStatistics", deserialize = "networkStatistics"))]
    NetworkStatistics(Box<crate::types::NetworkStatistics>),
}

impl NetworkStatistics {
    /// Convenience constructor to create a [`NetworkStatistics::NetworkStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn network_statistics(val: crate::types::NetworkStatistics) -> Self {
        Self::NetworkStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::NetworkStatistics`] into [`NetworkStatistics`].
impl From<crate::types::NetworkStatistics> for NetworkStatistics {
    fn from(val: crate::types::NetworkStatistics) -> Self {
        Self::NetworkStatistics(Box::new(val))
    }
}

/// TDLib `AutoDownloadSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutoDownloadSettings {
    /// Contains auto-download settings
    #[serde(rename(serialize = "autoDownloadSettings", deserialize = "autoDownloadSettings"))]
    AutoDownloadSettings(Box<crate::types::AutoDownloadSettings>),
}

impl AutoDownloadSettings {
    /// Convenience constructor to create a [`AutoDownloadSettings::AutoDownloadSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn auto_download_settings(val: crate::types::AutoDownloadSettings) -> Self {
        Self::AutoDownloadSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::AutoDownloadSettings`] into [`AutoDownloadSettings`].
impl From<crate::types::AutoDownloadSettings> for AutoDownloadSettings {
    fn from(val: crate::types::AutoDownloadSettings) -> Self {
        Self::AutoDownloadSettings(Box::new(val))
    }
}

/// TDLib `AutoDownloadSettingsPresets` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutoDownloadSettingsPresets {
    /// Contains auto-download settings presets for the current user
    #[serde(rename(serialize = "autoDownloadSettingsPresets", deserialize = "autoDownloadSettingsPresets"))]
    AutoDownloadSettingsPresets(Box<crate::types::AutoDownloadSettingsPresets>),
}

impl AutoDownloadSettingsPresets {
    /// Convenience constructor to create a [`AutoDownloadSettingsPresets::AutoDownloadSettingsPresets`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn auto_download_settings_presets(val: crate::types::AutoDownloadSettingsPresets) -> Self {
        Self::AutoDownloadSettingsPresets(Box::new(val))
    }

}

/// Converts a [`crate::types::AutoDownloadSettingsPresets`] into [`AutoDownloadSettingsPresets`].
impl From<crate::types::AutoDownloadSettingsPresets> for AutoDownloadSettingsPresets {
    fn from(val: crate::types::AutoDownloadSettingsPresets) -> Self {
        Self::AutoDownloadSettingsPresets(Box::new(val))
    }
}

/// Describes scope of autosave settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettingsScope {
    /// Autosave settings applied to all private chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopePrivateChats", deserialize = "autosaveSettingsScopePrivateChats"))]
    PrivateChats,
    /// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopeGroupChats", deserialize = "autosaveSettingsScopeGroupChats"))]
    GroupChats,
    /// Autosave settings applied to all channel chats without chat-specific settings
    #[serde(rename(serialize = "autosaveSettingsScopeChannelChats", deserialize = "autosaveSettingsScopeChannelChats"))]
    ChannelChats,
    /// Autosave settings applied to a chat
    #[serde(rename(serialize = "autosaveSettingsScopeChat", deserialize = "autosaveSettingsScopeChat"))]
    Chat(Box<crate::types::AutosaveSettingsScopeChat>),
}

impl AutosaveSettingsScope {
    /// Convenience constructor to create a [`AutosaveSettingsScope::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::AutosaveSettingsScopeChat) -> Self {
        Self::Chat(Box::new(val))
    }

}

/// Converts a [`crate::types::AutosaveSettingsScopeChat`] into [`AutosaveSettingsScope`].
impl From<crate::types::AutosaveSettingsScopeChat> for AutosaveSettingsScope {
    fn from(val: crate::types::AutosaveSettingsScopeChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// TDLib `ScopeAutosaveSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ScopeAutosaveSettings {
    /// Contains autosave settings for an autosave settings scope
    #[serde(rename(serialize = "scopeAutosaveSettings", deserialize = "scopeAutosaveSettings"))]
    ScopeAutosaveSettings(Box<crate::types::ScopeAutosaveSettings>),
}

impl ScopeAutosaveSettings {
    /// Convenience constructor to create a [`ScopeAutosaveSettings::ScopeAutosaveSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn scope_autosave_settings(val: crate::types::ScopeAutosaveSettings) -> Self {
        Self::ScopeAutosaveSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::ScopeAutosaveSettings`] into [`ScopeAutosaveSettings`].
impl From<crate::types::ScopeAutosaveSettings> for ScopeAutosaveSettings {
    fn from(val: crate::types::ScopeAutosaveSettings) -> Self {
        Self::ScopeAutosaveSettings(Box::new(val))
    }
}

/// TDLib `AutosaveSettingsException` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettingsException {
    /// Contains autosave settings for a chat, which overrides default settings for the corresponding scope
    #[serde(rename(serialize = "autosaveSettingsException", deserialize = "autosaveSettingsException"))]
    AutosaveSettingsException(Box<crate::types::AutosaveSettingsException>),
}

impl AutosaveSettingsException {
    /// Convenience constructor to create a [`AutosaveSettingsException::AutosaveSettingsException`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn autosave_settings_exception(val: crate::types::AutosaveSettingsException) -> Self {
        Self::AutosaveSettingsException(Box::new(val))
    }

}

/// Converts a [`crate::types::AutosaveSettingsException`] into [`AutosaveSettingsException`].
impl From<crate::types::AutosaveSettingsException> for AutosaveSettingsException {
    fn from(val: crate::types::AutosaveSettingsException) -> Self {
        Self::AutosaveSettingsException(Box::new(val))
    }
}

/// TDLib `AutosaveSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AutosaveSettings {
    /// Describes autosave settings
    #[serde(rename(serialize = "autosaveSettings", deserialize = "autosaveSettings"))]
    AutosaveSettings(Box<crate::types::AutosaveSettings>),
}

impl AutosaveSettings {
    /// Convenience constructor to create a [`AutosaveSettings::AutosaveSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn autosave_settings(val: crate::types::AutosaveSettings) -> Self {
        Self::AutosaveSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::AutosaveSettings`] into [`AutosaveSettings`].
impl From<crate::types::AutosaveSettings> for AutosaveSettings {
    fn from(val: crate::types::AutosaveSettings) -> Self {
        Self::AutosaveSettings(Box::new(val))
    }
}

/// TDLib `WebDomainException` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebDomainException {
    /// Describes an exception for built-in browser usage
    #[serde(rename(serialize = "webDomainException", deserialize = "webDomainException"))]
    WebDomainException(Box<crate::types::WebDomainException>),
}

impl WebDomainException {
    /// Convenience constructor to create a [`WebDomainException::WebDomainException`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_domain_exception(val: crate::types::WebDomainException) -> Self {
        Self::WebDomainException(Box::new(val))
    }

}

/// Converts a [`crate::types::WebDomainException`] into [`WebDomainException`].
impl From<crate::types::WebDomainException> for WebDomainException {
    fn from(val: crate::types::WebDomainException) -> Self {
        Self::WebDomainException(Box::new(val))
    }
}

/// TDLib `WebBrowserSettings` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebBrowserSettings {
    /// Describes web browser settings
    #[serde(rename(serialize = "webBrowserSettings", deserialize = "webBrowserSettings"))]
    WebBrowserSettings(Box<crate::types::WebBrowserSettings>),
}

impl WebBrowserSettings {
    /// Convenience constructor to create a [`WebBrowserSettings::WebBrowserSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_browser_settings(val: crate::types::WebBrowserSettings) -> Self {
        Self::WebBrowserSettings(Box::new(val))
    }

}

/// Converts a [`crate::types::WebBrowserSettings`] into [`WebBrowserSettings`].
impl From<crate::types::WebBrowserSettings> for WebBrowserSettings {
    fn from(val: crate::types::WebBrowserSettings) -> Self {
        Self::WebBrowserSettings(Box::new(val))
    }
}

/// Describes the type of web browser
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebBrowserType {
    /// An external web browser
    #[serde(rename(serialize = "webBrowserTypeExternal", deserialize = "webBrowserTypeExternal"))]
    External,
    /// The in-app browser
    #[serde(rename(serialize = "webBrowserTypeInApp", deserialize = "webBrowserTypeInApp"))]
    InApp,
}

/// Describes the current state of the connection to Telegram servers
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ConnectionState {
    /// Waiting for the network to become available. Use setNetworkType to change the available network type
    #[serde(rename(serialize = "connectionStateWaitingForNetwork", deserialize = "connectionStateWaitingForNetwork"))]
    WaitingForNetwork,
    /// Establishing a connection with a proxy server
    #[serde(rename(serialize = "connectionStateConnectingToProxy", deserialize = "connectionStateConnectingToProxy"))]
    ConnectingToProxy,
    /// Establishing a connection to the Telegram servers
    #[serde(rename(serialize = "connectionStateConnecting", deserialize = "connectionStateConnecting"))]
    Connecting,
    /// Downloading data expected to be received while the application was offline
    #[serde(rename(serialize = "connectionStateUpdating", deserialize = "connectionStateUpdating"))]
    Updating,
    /// There is a working connection to the Telegram servers
    #[serde(rename(serialize = "connectionStateReady", deserialize = "connectionStateReady"))]
    Ready,
}

/// TDLib `AgeVerificationParameters` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AgeVerificationParameters {
    /// Describes parameters for age verification of the current user
    #[serde(rename(serialize = "ageVerificationParameters", deserialize = "ageVerificationParameters"))]
    AgeVerificationParameters(Box<crate::types::AgeVerificationParameters>),
}

impl AgeVerificationParameters {
    /// Convenience constructor to create a [`AgeVerificationParameters::AgeVerificationParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn age_verification_parameters(val: crate::types::AgeVerificationParameters) -> Self {
        Self::AgeVerificationParameters(Box::new(val))
    }

}

/// Converts a [`crate::types::AgeVerificationParameters`] into [`AgeVerificationParameters`].
impl From<crate::types::AgeVerificationParameters> for AgeVerificationParameters {
    fn from(val: crate::types::AgeVerificationParameters) -> Self {
        Self::AgeVerificationParameters(Box::new(val))
    }
}

/// TDLib `FoundPosition` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPosition {
    /// Contains 0-based match position
    #[serde(rename(serialize = "foundPosition", deserialize = "foundPosition"))]
    FoundPosition(Box<crate::types::FoundPosition>),
}

impl FoundPosition {
    /// Convenience constructor to create a [`FoundPosition::FoundPosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_position(val: crate::types::FoundPosition) -> Self {
        Self::FoundPosition(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundPosition`] into [`FoundPosition`].
impl From<crate::types::FoundPosition> for FoundPosition {
    fn from(val: crate::types::FoundPosition) -> Self {
        Self::FoundPosition(Box::new(val))
    }
}

/// TDLib `FoundPositions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundPositions {
    /// Contains 0-based positions of matched objects
    #[serde(rename(serialize = "foundPositions", deserialize = "foundPositions"))]
    FoundPositions(Box<crate::types::FoundPositions>),
}

impl FoundPositions {
    /// Convenience constructor to create a [`FoundPositions::FoundPositions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_positions(val: crate::types::FoundPositions) -> Self {
        Self::FoundPositions(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundPositions`] into [`FoundPositions`].
impl From<crate::types::FoundPositions> for FoundPositions {
    fn from(val: crate::types::FoundPositions) -> Self {
        Self::FoundPositions(Box::new(val))
    }
}

/// Describes the type of URL linking to an internal Telegram entity
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrlType {
    /// A URL linking to a user
    #[serde(rename(serialize = "tMeUrlTypeUser", deserialize = "tMeUrlTypeUser"))]
    User(Box<crate::types::TmeUrlTypeUser>),
    /// A URL linking to a public supergroup or channel
    #[serde(rename(serialize = "tMeUrlTypeSupergroup", deserialize = "tMeUrlTypeSupergroup"))]
    Supergroup(Box<crate::types::TmeUrlTypeSupergroup>),
    /// A chat invite link
    #[serde(rename(serialize = "tMeUrlTypeChatInvite", deserialize = "tMeUrlTypeChatInvite"))]
    ChatInvite(Box<crate::types::TmeUrlTypeChatInvite>),
    /// A URL linking to a sticker set
    #[serde(rename(serialize = "tMeUrlTypeStickerSet", deserialize = "tMeUrlTypeStickerSet"))]
    StickerSet(Box<crate::types::TmeUrlTypeStickerSet>),
}

impl TmeUrlType {
    /// Convenience constructor to create a [`TmeUrlType::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::TmeUrlTypeUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`TmeUrlType::Supergroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup(val: crate::types::TmeUrlTypeSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }

    /// Convenience constructor to create a [`TmeUrlType::ChatInvite`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_invite(val: crate::types::TmeUrlTypeChatInvite) -> Self {
        Self::ChatInvite(Box::new(val))
    }

    /// Convenience constructor to create a [`TmeUrlType::StickerSet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set(val: crate::types::TmeUrlTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }

}

/// Converts a [`crate::types::TmeUrlTypeUser`] into [`TmeUrlType`].
impl From<crate::types::TmeUrlTypeUser> for TmeUrlType {
    fn from(val: crate::types::TmeUrlTypeUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::TmeUrlTypeSupergroup`] into [`TmeUrlType`].
impl From<crate::types::TmeUrlTypeSupergroup> for TmeUrlType {
    fn from(val: crate::types::TmeUrlTypeSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }
}

/// Converts a [`crate::types::TmeUrlTypeChatInvite`] into [`TmeUrlType`].
impl From<crate::types::TmeUrlTypeChatInvite> for TmeUrlType {
    fn from(val: crate::types::TmeUrlTypeChatInvite) -> Self {
        Self::ChatInvite(Box::new(val))
    }
}

/// Converts a [`crate::types::TmeUrlTypeStickerSet`] into [`TmeUrlType`].
impl From<crate::types::TmeUrlTypeStickerSet> for TmeUrlType {
    fn from(val: crate::types::TmeUrlTypeStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }
}

/// TDLib `TMeUrl` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrl {
    /// Represents a URL linking to an internal Telegram entity
    #[serde(rename(serialize = "tMeUrl", deserialize = "tMeUrl"))]
    TmeUrl(Box<crate::types::TmeUrl>),
}

impl TmeUrl {
    /// Convenience constructor to create a [`TmeUrl::TmeUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn tme_url(val: crate::types::TmeUrl) -> Self {
        Self::TmeUrl(Box::new(val))
    }

}

/// Converts a [`crate::types::TmeUrl`] into [`TmeUrl`].
impl From<crate::types::TmeUrl> for TmeUrl {
    fn from(val: crate::types::TmeUrl) -> Self {
        Self::TmeUrl(Box::new(val))
    }
}

/// TDLib `TMeUrls` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrls {
    /// Contains a list of t.me URLs
    #[serde(rename(serialize = "tMeUrls", deserialize = "tMeUrls"))]
    TmeUrls(Box<crate::types::TmeUrls>),
}

impl TmeUrls {
    /// Convenience constructor to create a [`TmeUrls::TmeUrls`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn tme_urls(val: crate::types::TmeUrls) -> Self {
        Self::TmeUrls(Box::new(val))
    }

}

/// Converts a [`crate::types::TmeUrls`] into [`TmeUrls`].
impl From<crate::types::TmeUrls> for TmeUrls {
    fn from(val: crate::types::TmeUrls) -> Self {
        Self::TmeUrls(Box::new(val))
    }
}

/// Describes an action suggested to the current user
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedAction {
    /// Suggests the user to enable archive_and_mute_new_chats_from_unknown_users setting in archiveChatListSettings
    #[serde(rename(serialize = "suggestedActionEnableArchiveAndMuteNewChats", deserialize = "suggestedActionEnableArchiveAndMuteNewChats"))]
    EnableArchiveAndMuteNewChats,
    /// Suggests the user to check whether they still remember their 2-step verification password
    #[serde(rename(serialize = "suggestedActionCheckPassword", deserialize = "suggestedActionCheckPassword"))]
    CheckPassword,
    /// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
    #[serde(rename(serialize = "suggestedActionCheckPhoneNumber", deserialize = "suggestedActionCheckPhoneNumber"))]
    CheckPhoneNumber,
    /// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
    #[serde(rename(serialize = "suggestedActionViewChecksHint", deserialize = "suggestedActionViewChecksHint"))]
    ViewChecksHint,
    /// Suggests the user to convert specified supergroup to a broadcast group
    #[serde(rename(serialize = "suggestedActionConvertToBroadcastGroup", deserialize = "suggestedActionConvertToBroadcastGroup"))]
    ConvertToBroadcastGroup(Box<crate::types::SuggestedActionConvertToBroadcastGroup>),
    /// Suggests the user to set a 2-step verification password to be able to log in again
    #[serde(rename(serialize = "suggestedActionSetPassword", deserialize = "suggestedActionSetPassword"))]
    SetPassword(Box<crate::types::SuggestedActionSetPassword>),
    /// Suggests the user to upgrade the Premium subscription from monthly payments to annual payments
    #[serde(rename(serialize = "suggestedActionUpgradePremium", deserialize = "suggestedActionUpgradePremium"))]
    UpgradePremium,
    /// Suggests the user to restore a recently expired Premium subscription
    #[serde(rename(serialize = "suggestedActionRestorePremium", deserialize = "suggestedActionRestorePremium"))]
    RestorePremium,
    /// Suggests the user to subscribe to the Premium subscription with annual payments
    #[serde(rename(serialize = "suggestedActionSubscribeToAnnualPremium", deserialize = "suggestedActionSubscribeToAnnualPremium"))]
    SubscribeToAnnualPremium,
    /// Suggests the user to gift Telegram Premium to friends for Christmas
    #[serde(rename(serialize = "suggestedActionGiftPremiumForChristmas", deserialize = "suggestedActionGiftPremiumForChristmas"))]
    GiftPremiumForChristmas,
    /// Suggests the user to set birthdate
    #[serde(rename(serialize = "suggestedActionSetBirthdate", deserialize = "suggestedActionSetBirthdate"))]
    SetBirthdate,
    /// Suggests the user to set profile photo
    #[serde(rename(serialize = "suggestedActionSetProfilePhoto", deserialize = "suggestedActionSetProfilePhoto"))]
    SetProfilePhoto,
    /// Suggests the user to extend their expiring Telegram Premium subscription
    #[serde(rename(serialize = "suggestedActionExtendPremium", deserialize = "suggestedActionExtendPremium"))]
    ExtendPremium(Box<crate::types::SuggestedActionExtendPremium>),
    /// Suggests the user to extend their expiring Telegram Star subscriptions. Call getStarSubscriptions with only_expiring == true
    /// to get the number of expiring subscriptions and the number of required to buy Telegram Stars
    #[serde(rename(serialize = "suggestedActionExtendStarSubscriptions", deserialize = "suggestedActionExtendStarSubscriptions"))]
    ExtendStarSubscriptions,
    /// A custom suggestion to be shown at the top of the chat list
    #[serde(rename(serialize = "suggestedActionCustom", deserialize = "suggestedActionCustom"))]
    Custom(Box<crate::types::SuggestedActionCustom>),
    /// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
    #[serde(rename(serialize = "suggestedActionSetLoginEmailAddress", deserialize = "suggestedActionSetLoginEmailAddress"))]
    SetLoginEmailAddress(Box<crate::types::SuggestedActionSetLoginEmailAddress>),
    /// Suggests the user to add a passkey for login using addLoginPasskey
    #[serde(rename(serialize = "suggestedActionAddLoginPasskey", deserialize = "suggestedActionAddLoginPasskey"))]
    AddLoginPasskey,
}

impl SuggestedAction {
    /// Convenience constructor to create a [`SuggestedAction::ConvertToBroadcastGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn convert_to_broadcast_group(val: crate::types::SuggestedActionConvertToBroadcastGroup) -> Self {
        Self::ConvertToBroadcastGroup(Box::new(val))
    }

    /// Convenience constructor to create a [`SuggestedAction::SetPassword`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn set_password(val: crate::types::SuggestedActionSetPassword) -> Self {
        Self::SetPassword(Box::new(val))
    }

    /// Convenience constructor to create a [`SuggestedAction::ExtendPremium`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn extend_premium(val: crate::types::SuggestedActionExtendPremium) -> Self {
        Self::ExtendPremium(Box::new(val))
    }

    /// Convenience constructor to create a [`SuggestedAction::Custom`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom(val: crate::types::SuggestedActionCustom) -> Self {
        Self::Custom(Box::new(val))
    }

    /// Convenience constructor to create a [`SuggestedAction::SetLoginEmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn set_login_email_address(val: crate::types::SuggestedActionSetLoginEmailAddress) -> Self {
        Self::SetLoginEmailAddress(Box::new(val))
    }

}

/// Converts a [`crate::types::SuggestedActionConvertToBroadcastGroup`] into [`SuggestedAction`].
impl From<crate::types::SuggestedActionConvertToBroadcastGroup> for SuggestedAction {
    fn from(val: crate::types::SuggestedActionConvertToBroadcastGroup) -> Self {
        Self::ConvertToBroadcastGroup(Box::new(val))
    }
}

/// Converts a [`crate::types::SuggestedActionSetPassword`] into [`SuggestedAction`].
impl From<crate::types::SuggestedActionSetPassword> for SuggestedAction {
    fn from(val: crate::types::SuggestedActionSetPassword) -> Self {
        Self::SetPassword(Box::new(val))
    }
}

/// Converts a [`crate::types::SuggestedActionExtendPremium`] into [`SuggestedAction`].
impl From<crate::types::SuggestedActionExtendPremium> for SuggestedAction {
    fn from(val: crate::types::SuggestedActionExtendPremium) -> Self {
        Self::ExtendPremium(Box::new(val))
    }
}

/// Converts a [`crate::types::SuggestedActionCustom`] into [`SuggestedAction`].
impl From<crate::types::SuggestedActionCustom> for SuggestedAction {
    fn from(val: crate::types::SuggestedActionCustom) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Converts a [`crate::types::SuggestedActionSetLoginEmailAddress`] into [`SuggestedAction`].
impl From<crate::types::SuggestedActionSetLoginEmailAddress> for SuggestedAction {
    fn from(val: crate::types::SuggestedActionSetLoginEmailAddress) -> Self {
        Self::SetLoginEmailAddress(Box::new(val))
    }
}

/// TDLib `Count` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Count {
    /// Contains a counter
    #[serde(rename(serialize = "count", deserialize = "count"))]
    Count(Box<crate::types::Count>),
}

impl Count {
    /// Convenience constructor to create a [`Count::Count`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn count(val: crate::types::Count) -> Self {
        Self::Count(Box::new(val))
    }

}

/// Converts a [`crate::types::Count`] into [`Count`].
impl From<crate::types::Count> for Count {
    fn from(val: crate::types::Count) -> Self {
        Self::Count(Box::new(val))
    }
}

/// TDLib `Data` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Data {
    /// Contains some binary data
    #[serde(rename(serialize = "data", deserialize = "data"))]
    Data(Box<crate::types::Data>),
}

impl Data {
    /// Convenience constructor to create a [`Data::Data`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data(val: crate::types::Data) -> Self {
        Self::Data(Box::new(val))
    }

}

/// Converts a [`crate::types::Data`] into [`Data`].
impl From<crate::types::Data> for Data {
    fn from(val: crate::types::Data) -> Self {
        Self::Data(Box::new(val))
    }
}

/// TDLib `Seconds` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Seconds {
    /// Contains a value representing a number of seconds
    #[serde(rename(serialize = "seconds", deserialize = "seconds"))]
    Seconds(Box<crate::types::Seconds>),
}

impl Seconds {
    /// Convenience constructor to create a [`Seconds::Seconds`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn seconds(val: crate::types::Seconds) -> Self {
        Self::Seconds(Box::new(val))
    }

}

/// Converts a [`crate::types::Seconds`] into [`Seconds`].
impl From<crate::types::Seconds> for Seconds {
    fn from(val: crate::types::Seconds) -> Self {
        Self::Seconds(Box::new(val))
    }
}

/// TDLib `StarCount` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarCount {
    /// Contains a number of Telegram Stars
    #[serde(rename(serialize = "starCount", deserialize = "starCount"))]
    StarCount(Box<crate::types::StarCount>),
}

impl StarCount {
    /// Convenience constructor to create a [`StarCount::StarCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_count(val: crate::types::StarCount) -> Self {
        Self::StarCount(Box::new(val))
    }

}

/// Converts a [`crate::types::StarCount`] into [`StarCount`].
impl From<crate::types::StarCount> for StarCount {
    fn from(val: crate::types::StarCount) -> Self {
        Self::StarCount(Box::new(val))
    }
}

/// TDLib `DeepLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DeepLinkInfo {
    /// Contains information about a tg: deep link
    #[serde(rename(serialize = "deepLinkInfo", deserialize = "deepLinkInfo"))]
    DeepLinkInfo(Box<crate::types::DeepLinkInfo>),
}

impl DeepLinkInfo {
    /// Convenience constructor to create a [`DeepLinkInfo::DeepLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn deep_link_info(val: crate::types::DeepLinkInfo) -> Self {
        Self::DeepLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::DeepLinkInfo`] into [`DeepLinkInfo`].
impl From<crate::types::DeepLinkInfo> for DeepLinkInfo {
    fn from(val: crate::types::DeepLinkInfo) -> Self {
        Self::DeepLinkInfo(Box::new(val))
    }
}

/// Describes the type of proxy server
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProxyType {
    /// A SOCKS5 proxy server
    #[serde(rename(serialize = "proxyTypeSocks5", deserialize = "proxyTypeSocks5"))]
    Socks5(Box<crate::types::ProxyTypeSocks5>),
    /// A HTTP transparent proxy server
    #[serde(rename(serialize = "proxyTypeHttp", deserialize = "proxyTypeHttp"))]
    Http(Box<crate::types::ProxyTypeHttp>),
    /// An MTProto proxy server
    #[serde(rename(serialize = "proxyTypeMtproto", deserialize = "proxyTypeMtproto"))]
    Mtproto(Box<crate::types::ProxyTypeMtproto>),
}

impl ProxyType {
    /// Convenience constructor to create a [`ProxyType::Socks5`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn socks_5(val: crate::types::ProxyTypeSocks5) -> Self {
        Self::Socks5(Box::new(val))
    }

    /// Convenience constructor to create a [`ProxyType::Http`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn http(val: crate::types::ProxyTypeHttp) -> Self {
        Self::Http(Box::new(val))
    }

    /// Convenience constructor to create a [`ProxyType::Mtproto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mtproto(val: crate::types::ProxyTypeMtproto) -> Self {
        Self::Mtproto(Box::new(val))
    }

}

/// Converts a [`crate::types::ProxyTypeSocks5`] into [`ProxyType`].
impl From<crate::types::ProxyTypeSocks5> for ProxyType {
    fn from(val: crate::types::ProxyTypeSocks5) -> Self {
        Self::Socks5(Box::new(val))
    }
}

/// Converts a [`crate::types::ProxyTypeHttp`] into [`ProxyType`].
impl From<crate::types::ProxyTypeHttp> for ProxyType {
    fn from(val: crate::types::ProxyTypeHttp) -> Self {
        Self::Http(Box::new(val))
    }
}

/// Converts a [`crate::types::ProxyTypeMtproto`] into [`ProxyType`].
impl From<crate::types::ProxyTypeMtproto> for ProxyType {
    fn from(val: crate::types::ProxyTypeMtproto) -> Self {
        Self::Mtproto(Box::new(val))
    }
}

/// TDLib `AddedProxy` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedProxy {
    /// Contains information about a proxy server added to the list of proxies
    #[serde(rename(serialize = "addedProxy", deserialize = "addedProxy"))]
    AddedProxy(Box<crate::types::AddedProxy>),
}

impl AddedProxy {
    /// Convenience constructor to create a [`AddedProxy::AddedProxy`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn added_proxy(val: crate::types::AddedProxy) -> Self {
        Self::AddedProxy(Box::new(val))
    }

}

/// Converts a [`crate::types::AddedProxy`] into [`AddedProxy`].
impl From<crate::types::AddedProxy> for AddedProxy {
    fn from(val: crate::types::AddedProxy) -> Self {
        Self::AddedProxy(Box::new(val))
    }
}

/// TDLib `AddedProxies` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedProxies {
    /// Represents a list of added proxy servers
    #[serde(rename(serialize = "addedProxies", deserialize = "addedProxies"))]
    AddedProxies(Box<crate::types::AddedProxies>),
}

impl AddedProxies {
    /// Convenience constructor to create a [`AddedProxies::AddedProxies`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn added_proxies(val: crate::types::AddedProxies) -> Self {
        Self::AddedProxies(Box::new(val))
    }

}

/// Converts a [`crate::types::AddedProxies`] into [`AddedProxies`].
impl From<crate::types::AddedProxies> for AddedProxies {
    fn from(val: crate::types::AddedProxies) -> Self {
        Self::AddedProxies(Box::new(val))
    }
}

/// TDLib `DateRange` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DateRange {
    /// Represents a date range
    #[serde(rename(serialize = "dateRange", deserialize = "dateRange"))]
    DateRange(Box<crate::types::DateRange>),
}

impl DateRange {
    /// Convenience constructor to create a [`DateRange::DateRange`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn date_range(val: crate::types::DateRange) -> Self {
        Self::DateRange(Box::new(val))
    }

}

/// Converts a [`crate::types::DateRange`] into [`DateRange`].
impl From<crate::types::DateRange> for DateRange {
    fn from(val: crate::types::DateRange) -> Self {
        Self::DateRange(Box::new(val))
    }
}

/// TDLib `StatisticalValue` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StatisticalValue {
    /// A value with information about its recent changes
    #[serde(rename(serialize = "statisticalValue", deserialize = "statisticalValue"))]
    StatisticalValue(Box<crate::types::StatisticalValue>),
}

impl StatisticalValue {
    /// Convenience constructor to create a [`StatisticalValue::StatisticalValue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn statistical_value(val: crate::types::StatisticalValue) -> Self {
        Self::StatisticalValue(Box::new(val))
    }

}

/// Converts a [`crate::types::StatisticalValue`] into [`StatisticalValue`].
impl From<crate::types::StatisticalValue> for StatisticalValue {
    fn from(val: crate::types::StatisticalValue) -> Self {
        Self::StatisticalValue(Box::new(val))
    }
}

/// Describes a statistical graph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StatisticalGraph {
    /// A graph data
    #[serde(rename(serialize = "statisticalGraphData", deserialize = "statisticalGraphData"))]
    Data(Box<crate::types::StatisticalGraphData>),
    /// The graph data to be asynchronously loaded through getStatisticalGraph
    #[serde(rename(serialize = "statisticalGraphAsync", deserialize = "statisticalGraphAsync"))]
    Async(Box<crate::types::StatisticalGraphAsync>),
    /// An error message to be shown to the user instead of the graph
    #[serde(rename(serialize = "statisticalGraphError", deserialize = "statisticalGraphError"))]
    Error(Box<crate::types::StatisticalGraphError>),
}

impl StatisticalGraph {
    /// Convenience constructor to create a [`StatisticalGraph::Data`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data(val: crate::types::StatisticalGraphData) -> Self {
        Self::Data(Box::new(val))
    }

    /// Convenience constructor to create a [`StatisticalGraph::Async`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn r#async(val: crate::types::StatisticalGraphAsync) -> Self {
        Self::Async(Box::new(val))
    }

    /// Convenience constructor to create a [`StatisticalGraph::Error`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn error(val: crate::types::StatisticalGraphError) -> Self {
        Self::Error(Box::new(val))
    }

}

/// Converts a [`crate::types::StatisticalGraphData`] into [`StatisticalGraph`].
impl From<crate::types::StatisticalGraphData> for StatisticalGraph {
    fn from(val: crate::types::StatisticalGraphData) -> Self {
        Self::Data(Box::new(val))
    }
}

/// Converts a [`crate::types::StatisticalGraphAsync`] into [`StatisticalGraph`].
impl From<crate::types::StatisticalGraphAsync> for StatisticalGraph {
    fn from(val: crate::types::StatisticalGraphAsync) -> Self {
        Self::Async(Box::new(val))
    }
}

/// Converts a [`crate::types::StatisticalGraphError`] into [`StatisticalGraph`].
impl From<crate::types::StatisticalGraphError> for StatisticalGraph {
    fn from(val: crate::types::StatisticalGraphError) -> Self {
        Self::Error(Box::new(val))
    }
}

/// TDLib `Point` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Point {
    /// A point on a Cartesian plane
    #[serde(rename(serialize = "point", deserialize = "point"))]
    Point(Box<crate::types::Point>),
}

impl Point {
    /// Convenience constructor to create a [`Point::Point`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn point(val: crate::types::Point) -> Self {
        Self::Point(Box::new(val))
    }

}

/// Converts a [`crate::types::Point`] into [`Point`].
impl From<crate::types::Point> for Point {
    fn from(val: crate::types::Point) -> Self {
        Self::Point(Box::new(val))
    }
}

/// Represents a vector path command
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VectorPathCommand {
    /// A straight line to a given point
    #[serde(rename(serialize = "vectorPathCommandLine", deserialize = "vectorPathCommandLine"))]
    Line(Box<crate::types::VectorPathCommandLine>),
    /// A cubic Bézier curve to a given point
    #[serde(rename(serialize = "vectorPathCommandCubicBezierCurve", deserialize = "vectorPathCommandCubicBezierCurve"))]
    CubicBezierCurve(Box<crate::types::VectorPathCommandCubicBezierCurve>),
}

impl VectorPathCommand {
    /// Convenience constructor to create a [`VectorPathCommand::Line`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn line(val: crate::types::VectorPathCommandLine) -> Self {
        Self::Line(Box::new(val))
    }

    /// Convenience constructor to create a [`VectorPathCommand::CubicBezierCurve`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn cubic_bezier_curve(val: crate::types::VectorPathCommandCubicBezierCurve) -> Self {
        Self::CubicBezierCurve(Box::new(val))
    }

}

/// Converts a [`crate::types::VectorPathCommandLine`] into [`VectorPathCommand`].
impl From<crate::types::VectorPathCommandLine> for VectorPathCommand {
    fn from(val: crate::types::VectorPathCommandLine) -> Self {
        Self::Line(Box::new(val))
    }
}

/// Converts a [`crate::types::VectorPathCommandCubicBezierCurve`] into [`VectorPathCommand`].
impl From<crate::types::VectorPathCommandCubicBezierCurve> for VectorPathCommand {
    fn from(val: crate::types::VectorPathCommandCubicBezierCurve) -> Self {
        Self::CubicBezierCurve(Box::new(val))
    }
}

/// Describes type of the request for which a code is sent to a phone number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberCodeType {
    /// Checks ownership of a new phone number to change the user's authentication phone number; for official Android and iOS applications only
    #[serde(rename(serialize = "phoneNumberCodeTypeChange", deserialize = "phoneNumberCodeTypeChange"))]
    Change,
    /// Verifies ownership of a phone number to be added to the user's Telegram Passport
    #[serde(rename(serialize = "phoneNumberCodeTypeVerify", deserialize = "phoneNumberCodeTypeVerify"))]
    Verify,
    /// Confirms ownership of a phone number to prevent account deletion while handling links of the type internalLinkTypePhoneNumberConfirmation
    #[serde(rename(serialize = "phoneNumberCodeTypeConfirmOwnership", deserialize = "phoneNumberCodeTypeConfirmOwnership"))]
    ConfirmOwnership(Box<crate::types::PhoneNumberCodeTypeConfirmOwnership>),
}

impl PhoneNumberCodeType {
    /// Convenience constructor to create a [`PhoneNumberCodeType::ConfirmOwnership`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn confirm_ownership(val: crate::types::PhoneNumberCodeTypeConfirmOwnership) -> Self {
        Self::ConfirmOwnership(Box::new(val))
    }

}

/// Converts a [`crate::types::PhoneNumberCodeTypeConfirmOwnership`] into [`PhoneNumberCodeType`].
impl From<crate::types::PhoneNumberCodeTypeConfirmOwnership> for PhoneNumberCodeType {
    fn from(val: crate::types::PhoneNumberCodeTypeConfirmOwnership) -> Self {
        Self::ConfirmOwnership(Box::new(val))
    }
}

/// Contains notifications about data changes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Update {
    /// The user authorization state has changed
    #[serde(rename(serialize = "updateAuthorizationState", deserialize = "updateAuthorizationState"))]
    AuthorizationState(Box<crate::types::UpdateAuthorizationState>),
    /// A new message was received; can also be an outgoing message
    #[serde(rename(serialize = "updateNewMessage", deserialize = "updateNewMessage"))]
    NewMessage(Box<crate::types::UpdateNewMessage>),
    /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully.
    /// This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
    #[serde(rename(serialize = "updateMessageSendAcknowledged", deserialize = "updateMessageSendAcknowledged"))]
    MessageSendAcknowledged(Box<crate::types::UpdateMessageSendAcknowledged>),
    /// A message has been successfully sent
    #[serde(rename(serialize = "updateMessageSendSucceeded", deserialize = "updateMessageSendSucceeded"))]
    MessageSendSucceeded(Box<crate::types::UpdateMessageSendSucceeded>),
    /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
    #[serde(rename(serialize = "updateMessageSendFailed", deserialize = "updateMessageSendFailed"))]
    MessageSendFailed(Box<crate::types::UpdateMessageSendFailed>),
    /// The message content has changed
    #[serde(rename(serialize = "updateMessageContent", deserialize = "updateMessageContent"))]
    MessageContent(Box<crate::types::UpdateMessageContent>),
    /// The message ephemeral content has changed
    #[serde(rename(serialize = "updateMessageEphemeralContent", deserialize = "updateMessageEphemeralContent"))]
    MessageEphemeralContent(Box<crate::types::UpdateMessageEphemeralContent>),
    /// A message was edited. Changes in the message content will come in a separate updateMessageContent
    #[serde(rename(serialize = "updateMessageEdited", deserialize = "updateMessageEdited"))]
    MessageEdited(Box<crate::types::UpdateMessageEdited>),
    /// The message pinned state was changed
    #[serde(rename(serialize = "updateMessageIsPinned", deserialize = "updateMessageIsPinned"))]
    MessageIsPinned(Box<crate::types::UpdateMessageIsPinned>),
    /// The information about interactions with a message has changed
    #[serde(rename(serialize = "updateMessageInteractionInfo", deserialize = "updateMessageInteractionInfo"))]
    MessageInteractionInfo(Box<crate::types::UpdateMessageInteractionInfo>),
    /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
    #[serde(rename(serialize = "updateMessageContentOpened", deserialize = "updateMessageContentOpened"))]
    MessageContentOpened(Box<crate::types::UpdateMessageContentOpened>),
    /// A message with an unread mention was read
    #[serde(rename(serialize = "updateMessageMentionRead", deserialize = "updateMessageMentionRead"))]
    MessageMentionRead(Box<crate::types::UpdateMessageMentionRead>),
    /// The list of unread reactions added to a message was changed
    #[serde(rename(serialize = "updateMessageUnreadReactions", deserialize = "updateMessageUnreadReactions"))]
    MessageUnreadReactions(Box<crate::types::UpdateMessageUnreadReactions>),
    /// Unread votes were added or removed from a poll message
    #[serde(rename(serialize = "updateMessageContainsUnreadPollVotes", deserialize = "updateMessageContainsUnreadPollVotes"))]
    MessageContainsUnreadPollVotes(Box<crate::types::UpdateMessageContainsUnreadPollVotes>),
    /// A fact-check added to a message was changed
    #[serde(rename(serialize = "updateMessageFactCheck", deserialize = "updateMessageFactCheck"))]
    MessageFactCheck(Box<crate::types::UpdateMessageFactCheck>),
    /// Information about suggested post of a message was changed
    #[serde(rename(serialize = "updateMessageSuggestedPostInfo", deserialize = "updateMessageSuggestedPostInfo"))]
    MessageSuggestedPostInfo(Box<crate::types::UpdateMessageSuggestedPostInfo>),
    /// A message with a live location was viewed. When the update is received, the application is expected to update the live location
    #[serde(rename(serialize = "updateMessageLiveLocationViewed", deserialize = "updateMessageLiveLocationViewed"))]
    MessageLiveLocationViewed(Box<crate::types::UpdateMessageLiveLocationViewed>),
    /// An automatically scheduled message with video has been successfully sent after conversion
    #[serde(rename(serialize = "updateVideoPublished", deserialize = "updateVideoPublished"))]
    VideoPublished(Box<crate::types::UpdateVideoPublished>),
    /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
    #[serde(rename(serialize = "updateNewChat", deserialize = "updateNewChat"))]
    NewChat(Box<crate::types::UpdateNewChat>),
    /// The title of a chat was changed
    #[serde(rename(serialize = "updateChatTitle", deserialize = "updateChatTitle"))]
    ChatTitle(Box<crate::types::UpdateChatTitle>),
    /// A chat photo was changed
    #[serde(rename(serialize = "updateChatPhoto", deserialize = "updateChatPhoto"))]
    ChatPhoto(Box<crate::types::UpdateChatPhoto>),
    /// Chat accent colors have changed
    #[serde(rename(serialize = "updateChatAccentColors", deserialize = "updateChatAccentColors"))]
    ChatAccentColors(Box<crate::types::UpdateChatAccentColors>),
    /// Chat permissions were changed
    #[serde(rename(serialize = "updateChatPermissions", deserialize = "updateChatPermissions"))]
    ChatPermissions(Box<crate::types::UpdateChatPermissions>),
    /// The last message of a chat was changed
    #[serde(rename(serialize = "updateChatLastMessage", deserialize = "updateChatLastMessage"))]
    ChatLastMessage(Box<crate::types::UpdateChatLastMessage>),
    /// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
    #[serde(rename(serialize = "updateChatPosition", deserialize = "updateChatPosition"))]
    ChatPosition(Box<crate::types::UpdateChatPosition>),
    /// A chat was added to a chat list
    #[serde(rename(serialize = "updateChatAddedToList", deserialize = "updateChatAddedToList"))]
    ChatAddedToList(Box<crate::types::UpdateChatAddedToList>),
    /// A chat was removed from a chat list
    #[serde(rename(serialize = "updateChatRemovedFromList", deserialize = "updateChatRemovedFromList"))]
    ChatRemovedFromList(Box<crate::types::UpdateChatRemovedFromList>),
    /// Incoming messages were read or the number of unread messages has been changed
    #[serde(rename(serialize = "updateChatReadInbox", deserialize = "updateChatReadInbox"))]
    ChatReadInbox(Box<crate::types::UpdateChatReadInbox>),
    /// Outgoing messages were read
    #[serde(rename(serialize = "updateChatReadOutbox", deserialize = "updateChatReadOutbox"))]
    ChatReadOutbox(Box<crate::types::UpdateChatReadOutbox>),
    /// The chat action bar was changed
    #[serde(rename(serialize = "updateChatActionBar", deserialize = "updateChatActionBar"))]
    ChatActionBar(Box<crate::types::UpdateChatActionBar>),
    /// The bar for managing business bot was changed in a chat
    #[serde(rename(serialize = "updateChatBusinessBotManageBar", deserialize = "updateChatBusinessBotManageBar"))]
    ChatBusinessBotManageBar(Box<crate::types::UpdateChatBusinessBotManageBar>),
    /// The chat available reactions were changed
    #[serde(rename(serialize = "updateChatAvailableReactions", deserialize = "updateChatAvailableReactions"))]
    ChatAvailableReactions(Box<crate::types::UpdateChatAvailableReactions>),
    /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
    #[serde(rename(serialize = "updateChatDraftMessage", deserialize = "updateChatDraftMessage"))]
    ChatDraftMessage(Box<crate::types::UpdateChatDraftMessage>),
    /// Chat emoji status has changed
    #[serde(rename(serialize = "updateChatEmojiStatus", deserialize = "updateChatEmojiStatus"))]
    ChatEmojiStatus(Box<crate::types::UpdateChatEmojiStatus>),
    /// The message sender that is selected to send messages in a chat has changed
    #[serde(rename(serialize = "updateChatMessageSender", deserialize = "updateChatMessageSender"))]
    ChatMessageSender(Box<crate::types::UpdateChatMessageSender>),
    /// The message auto-delete or self-destruct timer setting for a chat was changed
    #[serde(rename(serialize = "updateChatMessageAutoDeleteTime", deserialize = "updateChatMessageAutoDeleteTime"))]
    ChatMessageAutoDeleteTime(Box<crate::types::UpdateChatMessageAutoDeleteTime>),
    /// Notification settings for a chat were changed
    #[serde(rename(serialize = "updateChatNotificationSettings", deserialize = "updateChatNotificationSettings"))]
    ChatNotificationSettings(Box<crate::types::UpdateChatNotificationSettings>),
    /// The chat pending join requests were changed
    #[serde(rename(serialize = "updateChatPendingJoinRequests", deserialize = "updateChatPendingJoinRequests"))]
    ChatPendingJoinRequests(Box<crate::types::UpdateChatPendingJoinRequests>),
    /// The chat reply markup was changed
    #[serde(rename(serialize = "updateChatReplyMarkup", deserialize = "updateChatReplyMarkup"))]
    ChatReplyMarkup(Box<crate::types::UpdateChatReplyMarkup>),
    /// The chat background was changed
    #[serde(rename(serialize = "updateChatBackground", deserialize = "updateChatBackground"))]
    ChatBackground(Box<crate::types::UpdateChatBackground>),
    /// The chat theme was changed
    #[serde(rename(serialize = "updateChatTheme", deserialize = "updateChatTheme"))]
    ChatTheme(Box<crate::types::UpdateChatTheme>),
    /// The chat unread_mention_count has changed
    #[serde(rename(serialize = "updateChatUnreadMentionCount", deserialize = "updateChatUnreadMentionCount"))]
    ChatUnreadMentionCount(Box<crate::types::UpdateChatUnreadMentionCount>),
    /// The chat unread_reaction_count has changed
    #[serde(rename(serialize = "updateChatUnreadReactionCount", deserialize = "updateChatUnreadReactionCount"))]
    ChatUnreadReactionCount(Box<crate::types::UpdateChatUnreadReactionCount>),
    /// The chat unread_poll_vote_count has changed
    #[serde(rename(serialize = "updateChatUnreadPollVoteCount", deserialize = "updateChatUnreadPollVoteCount"))]
    ChatUnreadPollVoteCount(Box<crate::types::UpdateChatUnreadPollVoteCount>),
    /// A chat video chat state has changed
    #[serde(rename(serialize = "updateChatVideoChat", deserialize = "updateChatVideoChat"))]
    ChatVideoChat(Box<crate::types::UpdateChatVideoChat>),
    /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
    #[serde(rename(serialize = "updateChatDefaultDisableNotification", deserialize = "updateChatDefaultDisableNotification"))]
    ChatDefaultDisableNotification(Box<crate::types::UpdateChatDefaultDisableNotification>),
    /// A chat content was allowed or restricted for saving
    #[serde(rename(serialize = "updateChatHasProtectedContent", deserialize = "updateChatHasProtectedContent"))]
    ChatHasProtectedContent(Box<crate::types::UpdateChatHasProtectedContent>),
    /// Translation of chat messages was enabled or disabled
    #[serde(rename(serialize = "updateChatIsTranslatable", deserialize = "updateChatIsTranslatable"))]
    ChatIsTranslatable(Box<crate::types::UpdateChatIsTranslatable>),
    /// A chat was marked as unread or was read
    #[serde(rename(serialize = "updateChatIsMarkedAsUnread", deserialize = "updateChatIsMarkedAsUnread"))]
    ChatIsMarkedAsUnread(Box<crate::types::UpdateChatIsMarkedAsUnread>),
    /// A chat default appearance has changed
    #[serde(rename(serialize = "updateChatViewAsTopics", deserialize = "updateChatViewAsTopics"))]
    ChatViewAsTopics(Box<crate::types::UpdateChatViewAsTopics>),
    /// A chat was blocked or unblocked
    #[serde(rename(serialize = "updateChatBlockList", deserialize = "updateChatBlockList"))]
    ChatBlockList(Box<crate::types::UpdateChatBlockList>),
    /// A chat's has_scheduled_messages field has changed
    #[serde(rename(serialize = "updateChatHasScheduledMessages", deserialize = "updateChatHasScheduledMessages"))]
    ChatHasScheduledMessages(Box<crate::types::UpdateChatHasScheduledMessages>),
    /// A chat's has_welcome_messages field has changed
    #[serde(rename(serialize = "updateChatHasWelcomeMessages", deserialize = "updateChatHasWelcomeMessages"))]
    ChatHasWelcomeMessages(Box<crate::types::UpdateChatHasWelcomeMessages>),
    /// The list of chat folders or a chat folder has changed
    #[serde(rename(serialize = "updateChatFolders", deserialize = "updateChatFolders"))]
    ChatFolders(Box<crate::types::UpdateChatFolders>),
    /// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats.
    /// There is no guarantee that it is sent just after the number of online users has changed
    #[serde(rename(serialize = "updateChatOnlineMemberCount", deserialize = "updateChatOnlineMemberCount"))]
    ChatOnlineMemberCount(Box<crate::types::UpdateChatOnlineMemberCount>),
    /// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
    #[serde(rename(serialize = "updateSavedMessagesTopic", deserialize = "updateSavedMessagesTopic"))]
    SavedMessagesTopic(Box<crate::types::UpdateSavedMessagesTopic>),
    /// Number of Saved Messages topics has changed
    #[serde(rename(serialize = "updateSavedMessagesTopicCount", deserialize = "updateSavedMessagesTopicCount"))]
    SavedMessagesTopicCount(Box<crate::types::UpdateSavedMessagesTopicCount>),
    /// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
    #[serde(rename(serialize = "updateDirectMessagesChatTopic", deserialize = "updateDirectMessagesChatTopic"))]
    DirectMessagesChatTopic(Box<crate::types::UpdateDirectMessagesChatTopic>),
    /// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
    #[serde(rename(serialize = "updateTopicMessageCount", deserialize = "updateTopicMessageCount"))]
    TopicMessageCount(Box<crate::types::UpdateTopicMessageCount>),
    /// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
    #[serde(rename(serialize = "updateQuickReplyShortcut", deserialize = "updateQuickReplyShortcut"))]
    QuickReplyShortcut(Box<crate::types::UpdateQuickReplyShortcut>),
    /// A quick reply shortcut and all its messages were deleted
    #[serde(rename(serialize = "updateQuickReplyShortcutDeleted", deserialize = "updateQuickReplyShortcutDeleted"))]
    QuickReplyShortcutDeleted(Box<crate::types::UpdateQuickReplyShortcutDeleted>),
    /// The list of quick reply shortcuts has changed
    #[serde(rename(serialize = "updateQuickReplyShortcuts", deserialize = "updateQuickReplyShortcuts"))]
    QuickReplyShortcuts(Box<crate::types::UpdateQuickReplyShortcuts>),
    /// The list of quick reply shortcut messages has changed
    #[serde(rename(serialize = "updateQuickReplyShortcutMessages", deserialize = "updateQuickReplyShortcutMessages"))]
    QuickReplyShortcutMessages(Box<crate::types::UpdateQuickReplyShortcutMessages>),
    /// The list of welcome messages of a chat has changed
    #[serde(rename(serialize = "updateChatWelcomeMessages", deserialize = "updateChatWelcomeMessages"))]
    ChatWelcomeMessages(Box<crate::types::UpdateChatWelcomeMessages>),
    /// Basic information about a topic in a forum chat was changed
    #[serde(rename(serialize = "updateForumTopicInfo", deserialize = "updateForumTopicInfo"))]
    ForumTopicInfo(Box<crate::types::UpdateForumTopicInfo>),
    /// Information about a topic in a forum chat was changed
    #[serde(rename(serialize = "updateForumTopic", deserialize = "updateForumTopic"))]
    ForumTopic(Box<crate::types::UpdateForumTopic>),
    /// Notification settings for some type of chats were updated
    #[serde(rename(serialize = "updateScopeNotificationSettings", deserialize = "updateScopeNotificationSettings"))]
    ScopeNotificationSettings(Box<crate::types::UpdateScopeNotificationSettings>),
    /// Notification settings for reactions were updated
    #[serde(rename(serialize = "updateReactionNotificationSettings", deserialize = "updateReactionNotificationSettings"))]
    ReactionNotificationSettings(Box<crate::types::UpdateReactionNotificationSettings>),
    /// A notification was changed
    #[serde(rename(serialize = "updateNotification", deserialize = "updateNotification"))]
    Notification(Box<crate::types::UpdateNotification>),
    /// A list of active notifications in a notification group has changed
    #[serde(rename(serialize = "updateNotificationGroup", deserialize = "updateNotificationGroup"))]
    NotificationGroup(Box<crate::types::UpdateNotificationGroup>),
    /// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
    #[serde(rename(serialize = "updateActiveNotifications", deserialize = "updateActiveNotifications"))]
    ActiveNotifications(Box<crate::types::UpdateActiveNotifications>),
    /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
    #[serde(rename(serialize = "updateHavePendingNotifications", deserialize = "updateHavePendingNotifications"))]
    HavePendingNotifications(Box<crate::types::UpdateHavePendingNotifications>),
    /// Some messages were deleted
    #[serde(rename(serialize = "updateDeleteMessages", deserialize = "updateDeleteMessages"))]
    DeleteMessages(Box<crate::types::UpdateDeleteMessages>),
    /// A message sender activity in the chat has changed
    #[serde(rename(serialize = "updateChatAction", deserialize = "updateChatAction"))]
    ChatAction(Box<crate::types::UpdateChatAction>),
    /// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds,
    /// replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
    #[serde(rename(serialize = "updatePendingMessage", deserialize = "updatePendingMessage"))]
    PendingMessage(Box<crate::types::UpdatePendingMessage>),
    /// A message draft generation was stopped by the user
    #[serde(rename(serialize = "updateStopMessageDraft", deserialize = "updateStopMessageDraft"))]
    StopMessageDraft(Box<crate::types::UpdateStopMessageDraft>),
    /// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
    #[serde(rename(serialize = "updateCommunity", deserialize = "updateCommunity"))]
    Community(Box<crate::types::UpdateCommunity>),
    /// The user went online or offline
    #[serde(rename(serialize = "updateUserStatus", deserialize = "updateUserStatus"))]
    UserStatus(Box<crate::types::UpdateUserStatus>),
    /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
    #[serde(rename(serialize = "updateUser", deserialize = "updateUser"))]
    User(Box<crate::types::UpdateUser>),
    /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
    #[serde(rename(serialize = "updateBasicGroup", deserialize = "updateBasicGroup"))]
    BasicGroup(Box<crate::types::UpdateBasicGroup>),
    /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
    #[serde(rename(serialize = "updateSupergroup", deserialize = "updateSupergroup"))]
    Supergroup(Box<crate::types::UpdateSupergroup>),
    /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
    #[serde(rename(serialize = "updateSecretChat", deserialize = "updateSecretChat"))]
    SecretChat(Box<crate::types::UpdateSecretChat>),
    /// Some data in userFullInfo has been changed
    #[serde(rename(serialize = "updateUserFullInfo", deserialize = "updateUserFullInfo"))]
    UserFullInfo(Box<crate::types::UpdateUserFullInfo>),
    /// Some data in basicGroupFullInfo has been changed
    #[serde(rename(serialize = "updateBasicGroupFullInfo", deserialize = "updateBasicGroupFullInfo"))]
    BasicGroupFullInfo(Box<crate::types::UpdateBasicGroupFullInfo>),
    /// Some data in supergroupFullInfo has been changed
    #[serde(rename(serialize = "updateSupergroupFullInfo", deserialize = "updateSupergroupFullInfo"))]
    SupergroupFullInfo(Box<crate::types::UpdateSupergroupFullInfo>),
    /// Some data in communityFullInfo has been changed
    #[serde(rename(serialize = "updateCommunityFullInfo", deserialize = "updateCommunityFullInfo"))]
    CommunityFullInfo(Box<crate::types::UpdateCommunityFullInfo>),
    /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
    #[serde(rename(serialize = "updateServiceNotification", deserialize = "updateServiceNotification"))]
    ServiceNotification(Box<crate::types::UpdateServiceNotification>),
    /// An OAuth authorization request was received
    #[serde(rename(serialize = "updateNewOauthRequest", deserialize = "updateNewOauthRequest"))]
    NewOauthRequest(Box<crate::types::UpdateNewOauthRequest>),
    /// Information about a file was updated
    #[serde(rename(serialize = "updateFile", deserialize = "updateFile"))]
    File(Box<crate::types::UpdateFile>),
    /// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
    #[serde(rename(serialize = "updateFileGenerationStart", deserialize = "updateFileGenerationStart"))]
    FileGenerationStart(Box<crate::types::UpdateFileGenerationStart>),
    /// File generation is no longer needed
    #[serde(rename(serialize = "updateFileGenerationStop", deserialize = "updateFileGenerationStop"))]
    FileGenerationStop(Box<crate::types::UpdateFileGenerationStop>),
    /// The state of the file download list has changed
    #[serde(rename(serialize = "updateFileDownloads", deserialize = "updateFileDownloads"))]
    FileDownloads(Box<crate::types::UpdateFileDownloads>),
    /// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
    #[serde(rename(serialize = "updateFileAddedToDownloads", deserialize = "updateFileAddedToDownloads"))]
    FileAddedToDownloads(Box<crate::types::UpdateFileAddedToDownloads>),
    /// A file download was changed. This update is sent only after file download list is loaded for the first time
    #[serde(rename(serialize = "updateFileDownload", deserialize = "updateFileDownload"))]
    FileDownload(Box<crate::types::UpdateFileDownload>),
    /// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
    #[serde(rename(serialize = "updateFileRemovedFromDownloads", deserialize = "updateFileRemovedFromDownloads"))]
    FileRemovedFromDownloads(Box<crate::types::UpdateFileRemovedFromDownloads>),
    /// A request can't be completed unless application verification is performed; for official mobile applications only.
    /// The method setApplicationVerificationToken must be called once the verification is completed or failed
    #[serde(rename(serialize = "updateApplicationVerificationRequired", deserialize = "updateApplicationVerificationRequired"))]
    ApplicationVerificationRequired(Box<crate::types::UpdateApplicationVerificationRequired>),
    /// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only.
    /// The method setApplicationVerificationToken must be called once the verification is completed or failed
    #[serde(rename(serialize = "updateApplicationRecaptchaVerificationRequired", deserialize = "updateApplicationRecaptchaVerificationRequired"))]
    ApplicationRecaptchaVerificationRequired(Box<crate::types::UpdateApplicationRecaptchaVerificationRequired>),
    /// New call was created or information about a call was updated
    #[serde(rename(serialize = "updateCall", deserialize = "updateCall"))]
    Call(Box<crate::types::UpdateCall>),
    /// Information about a group call was updated
    #[serde(rename(serialize = "updateGroupCall", deserialize = "updateGroupCall"))]
    GroupCall(Box<crate::types::UpdateGroupCall>),
    /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
    #[serde(rename(serialize = "updateGroupCallParticipant", deserialize = "updateGroupCallParticipant"))]
    GroupCallParticipant(Box<crate::types::UpdateGroupCallParticipant>),
    /// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
    #[serde(rename(serialize = "updateGroupCallParticipants", deserialize = "updateGroupCallParticipants"))]
    GroupCallParticipants(Box<crate::types::UpdateGroupCallParticipants>),
    /// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
    #[serde(rename(serialize = "updateGroupCallVerificationState", deserialize = "updateGroupCallVerificationState"))]
    GroupCallVerificationState(Box<crate::types::UpdateGroupCallVerificationState>),
    /// A new message was received in a group call
    #[serde(rename(serialize = "updateNewGroupCallMessage", deserialize = "updateNewGroupCallMessage"))]
    NewGroupCallMessage(Box<crate::types::UpdateNewGroupCallMessage>),
    /// A new paid reaction was received in a live story group call
    #[serde(rename(serialize = "updateNewGroupCallPaidReaction", deserialize = "updateNewGroupCallPaidReaction"))]
    NewGroupCallPaidReaction(Box<crate::types::UpdateNewGroupCallPaidReaction>),
    /// A group call message failed to send
    #[serde(rename(serialize = "updateGroupCallMessageSendFailed", deserialize = "updateGroupCallMessageSendFailed"))]
    GroupCallMessageSendFailed(Box<crate::types::UpdateGroupCallMessageSendFailed>),
    /// Some group call messages were deleted
    #[serde(rename(serialize = "updateGroupCallMessagesDeleted", deserialize = "updateGroupCallMessagesDeleted"))]
    GroupCallMessagesDeleted(Box<crate::types::UpdateGroupCallMessagesDeleted>),
    /// The list of top donors in live story group call has changed
    #[serde(rename(serialize = "updateLiveStoryTopDonors", deserialize = "updateLiveStoryTopDonors"))]
    LiveStoryTopDonors(Box<crate::types::UpdateLiveStoryTopDonors>),
    /// New call signaling data arrived
    #[serde(rename(serialize = "updateNewCallSignalingData", deserialize = "updateNewCallSignalingData"))]
    NewCallSignalingData(Box<crate::types::UpdateNewCallSignalingData>),
    /// State of a gift auction was updated
    #[serde(rename(serialize = "updateGiftAuctionState", deserialize = "updateGiftAuctionState"))]
    GiftAuctionState(Box<crate::types::UpdateGiftAuctionState>),
    /// The list of auctions in which the current user participates has changed
    #[serde(rename(serialize = "updateActiveGiftAuctions", deserialize = "updateActiveGiftAuctions"))]
    ActiveGiftAuctions(Box<crate::types::UpdateActiveGiftAuctions>),
    /// Some privacy setting rules have been changed
    #[serde(rename(serialize = "updateUserPrivacySettingRules", deserialize = "updateUserPrivacySettingRules"))]
    UserPrivacySettingRules(Box<crate::types::UpdateUserPrivacySettingRules>),
    /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
    #[serde(rename(serialize = "updateUnreadMessageCount", deserialize = "updateUnreadMessageCount"))]
    UnreadMessageCount(Box<crate::types::UpdateUnreadMessageCount>),
    /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
    #[serde(rename(serialize = "updateUnreadChatCount", deserialize = "updateUnreadChatCount"))]
    UnreadChatCount(Box<crate::types::UpdateUnreadChatCount>),
    /// A join request from the user was completed
    #[serde(rename(serialize = "updateChatJoinResult", deserialize = "updateChatJoinResult"))]
    ChatJoinResult(Box<crate::types::UpdateChatJoinResult>),
    /// A story was changed
    #[serde(rename(serialize = "updateStory", deserialize = "updateStory"))]
    Story(Box<crate::types::UpdateStory>),
    /// A story became inaccessible
    #[serde(rename(serialize = "updateStoryDeleted", deserialize = "updateStoryDeleted"))]
    StoryDeleted(Box<crate::types::UpdateStoryDeleted>),
    /// A story has been successfully posted
    #[serde(rename(serialize = "updateStoryPostSucceeded", deserialize = "updateStoryPostSucceeded"))]
    StoryPostSucceeded(Box<crate::types::UpdateStoryPostSucceeded>),
    /// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
    #[serde(rename(serialize = "updateStoryPostFailed", deserialize = "updateStoryPostFailed"))]
    StoryPostFailed(Box<crate::types::UpdateStoryPostFailed>),
    /// The list of active stories posted by a specific chat has changed
    #[serde(rename(serialize = "updateChatActiveStories", deserialize = "updateChatActiveStories"))]
    ChatActiveStories(Box<crate::types::UpdateChatActiveStories>),
    /// Number of chats in a story list has changed
    #[serde(rename(serialize = "updateStoryListChatCount", deserialize = "updateStoryListChatCount"))]
    StoryListChatCount(Box<crate::types::UpdateStoryListChatCount>),
    /// Story stealth mode settings have changed
    #[serde(rename(serialize = "updateStoryStealthMode", deserialize = "updateStoryStealthMode"))]
    StoryStealthMode(Box<crate::types::UpdateStoryStealthMode>),
    /// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
    #[serde(rename(serialize = "updateTrustedMiniAppBots", deserialize = "updateTrustedMiniAppBots"))]
    TrustedMiniAppBots(Box<crate::types::UpdateTrustedMiniAppBots>),
    /// An option changed its value
    #[serde(rename(serialize = "updateOption", deserialize = "updateOption"))]
    Option(Box<crate::types::UpdateOption>),
    /// A sticker set has changed
    #[serde(rename(serialize = "updateStickerSet", deserialize = "updateStickerSet"))]
    StickerSet(Box<crate::types::UpdateStickerSet>),
    /// The list of installed sticker sets was updated
    #[serde(rename(serialize = "updateInstalledStickerSets", deserialize = "updateInstalledStickerSets"))]
    InstalledStickerSets(Box<crate::types::UpdateInstalledStickerSets>),
    /// The list of trending sticker sets was updated or some of them were viewed
    #[serde(rename(serialize = "updateTrendingStickerSets", deserialize = "updateTrendingStickerSets"))]
    TrendingStickerSets(Box<crate::types::UpdateTrendingStickerSets>),
    /// The list of recently used stickers was updated
    #[serde(rename(serialize = "updateRecentStickers", deserialize = "updateRecentStickers"))]
    RecentStickers(Box<crate::types::UpdateRecentStickers>),
    /// The list of favorite stickers was updated
    #[serde(rename(serialize = "updateFavoriteStickers", deserialize = "updateFavoriteStickers"))]
    FavoriteStickers(Box<crate::types::UpdateFavoriteStickers>),
    /// The list of saved animations was updated
    #[serde(rename(serialize = "updateSavedAnimations", deserialize = "updateSavedAnimations"))]
    SavedAnimations(Box<crate::types::UpdateSavedAnimations>),
    /// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
    #[serde(rename(serialize = "updateSavedNotificationSounds", deserialize = "updateSavedNotificationSounds"))]
    SavedNotificationSounds(Box<crate::types::UpdateSavedNotificationSounds>),
    /// The default background has changed
    #[serde(rename(serialize = "updateDefaultBackground", deserialize = "updateDefaultBackground"))]
    DefaultBackground(Box<crate::types::UpdateDefaultBackground>),
    /// The list of available emoji chat themes has changed
    #[serde(rename(serialize = "updateEmojiChatThemes", deserialize = "updateEmojiChatThemes"))]
    EmojiChatThemes(Box<crate::types::UpdateEmojiChatThemes>),
    /// The list of supported accent colors has changed
    #[serde(rename(serialize = "updateAccentColors", deserialize = "updateAccentColors"))]
    AccentColors(Box<crate::types::UpdateAccentColors>),
    /// The list of supported accent colors for user profiles has changed
    #[serde(rename(serialize = "updateProfileAccentColors", deserialize = "updateProfileAccentColors"))]
    ProfileAccentColors(Box<crate::types::UpdateProfileAccentColors>),
    /// Web browser settings have been updated
    #[serde(rename(serialize = "updateWebBrowserSettings", deserialize = "updateWebBrowserSettings"))]
    WebBrowserSettings(Box<crate::types::UpdateWebBrowserSettings>),
    /// Some language pack strings have been updated
    #[serde(rename(serialize = "updateLanguagePackStrings", deserialize = "updateLanguagePackStrings"))]
    LanguagePackStrings(Box<crate::types::UpdateLanguagePackStrings>),
    /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
    #[serde(rename(serialize = "updateConnectionState", deserialize = "updateConnectionState"))]
    ConnectionState(Box<crate::types::UpdateConnectionState>),
    /// The freeze state of the current user's account has changed
    #[serde(rename(serialize = "updateFreezeState", deserialize = "updateFreezeState"))]
    FreezeState(Box<crate::types::UpdateFreezeState>),
    /// The parameters for age verification of the current user's account have changed
    #[serde(rename(serialize = "updateAgeVerificationParameters", deserialize = "updateAgeVerificationParameters"))]
    AgeVerificationParameters(Box<crate::types::UpdateAgeVerificationParameters>),
    /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
    #[serde(rename(serialize = "updateTermsOfService", deserialize = "updateTermsOfService"))]
    TermsOfService(Box<crate::types::UpdateTermsOfService>),
    /// The first unconfirmed session has changed
    #[serde(rename(serialize = "updateUnconfirmedSession", deserialize = "updateUnconfirmedSession"))]
    UnconfirmedSession(Box<crate::types::UpdateUnconfirmedSession>),
    /// The list of bots added to attachment or side menu has changed
    #[serde(rename(serialize = "updateAttachmentMenuBots", deserialize = "updateAttachmentMenuBots"))]
    AttachmentMenuBots(Box<crate::types::UpdateAttachmentMenuBots>),
    /// A message was sent by an opened Web App, so the Web App needs to be closed
    #[serde(rename(serialize = "updateWebAppMessageSent", deserialize = "updateWebAppMessageSent"))]
    WebAppMessageSent(Box<crate::types::UpdateWebAppMessageSent>),
    /// The list of active emoji reactions has changed
    #[serde(rename(serialize = "updateActiveEmojiReactions", deserialize = "updateActiveEmojiReactions"))]
    ActiveEmojiReactions(Box<crate::types::UpdateActiveEmojiReactions>),
    /// The list of available message effects has changed
    #[serde(rename(serialize = "updateAvailableMessageEffects", deserialize = "updateAvailableMessageEffects"))]
    AvailableMessageEffects(Box<crate::types::UpdateAvailableMessageEffects>),
    /// The type of default reaction has changed
    #[serde(rename(serialize = "updateDefaultReactionType", deserialize = "updateDefaultReactionType"))]
    DefaultReactionType(Box<crate::types::UpdateDefaultReactionType>),
    /// The type of default paid reaction has changed
    #[serde(rename(serialize = "updateDefaultPaidReactionType", deserialize = "updateDefaultPaidReactionType"))]
    DefaultPaidReactionType(Box<crate::types::UpdateDefaultPaidReactionType>),
    /// Tags used in Saved Messages or a Saved Messages topic have changed
    #[serde(rename(serialize = "updateSavedMessagesTags", deserialize = "updateSavedMessagesTags"))]
    SavedMessagesTags(Box<crate::types::UpdateSavedMessagesTags>),
    /// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
    #[serde(rename(serialize = "updateActiveLiveLocationMessages", deserialize = "updateActiveLiveLocationMessages"))]
    ActiveLiveLocationMessages(Box<crate::types::UpdateActiveLiveLocationMessages>),
    /// The number of Telegram Stars owned by the current user has changed
    #[serde(rename(serialize = "updateOwnedStarCount", deserialize = "updateOwnedStarCount"))]
    OwnedStarCount(Box<crate::types::UpdateOwnedStarCount>),
    /// The number of TON Grams owned by the current user has changed
    #[serde(rename(serialize = "updateOwnedGramCount", deserialize = "updateOwnedGramCount"))]
    OwnedGramCount(Box<crate::types::UpdateOwnedGramCount>),
    /// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
    #[serde(rename(serialize = "updateChatRevenueAmount", deserialize = "updateChatRevenueAmount"))]
    ChatRevenueAmount(Box<crate::types::UpdateChatRevenueAmount>),
    /// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
    #[serde(rename(serialize = "updateStarRevenueStatus", deserialize = "updateStarRevenueStatus"))]
    StarRevenueStatus(Box<crate::types::UpdateStarRevenueStatus>),
    /// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
    #[serde(rename(serialize = "updateGramRevenueStatus", deserialize = "updateGramRevenueStatus"))]
    GramRevenueStatus(Box<crate::types::UpdateGramRevenueStatus>),
    /// The parameters of speech recognition without Telegram Premium subscription have changed
    #[serde(rename(serialize = "updateSpeechRecognitionTrial", deserialize = "updateSpeechRecognitionTrial"))]
    SpeechRecognitionTrial(Box<crate::types::UpdateSpeechRecognitionTrial>),
    /// The levels of live story group call messages have changed
    #[serde(rename(serialize = "updateGroupCallMessageLevels", deserialize = "updateGroupCallMessageLevels"))]
    GroupCallMessageLevels(Box<crate::types::UpdateGroupCallMessageLevels>),
    /// The list of supported dice emojis has changed
    #[serde(rename(serialize = "updateDiceEmojis", deserialize = "updateDiceEmojis"))]
    DiceEmojis(Box<crate::types::UpdateDiceEmojis>),
    /// The stake dice state has changed
    #[serde(rename(serialize = "updateStakeDiceState", deserialize = "updateStakeDiceState"))]
    StakeDiceState(Box<crate::types::UpdateStakeDiceState>),
    /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
    #[serde(rename(serialize = "updateAnimatedEmojiMessageClicked", deserialize = "updateAnimatedEmojiMessageClicked"))]
    AnimatedEmojiMessageClicked(Box<crate::types::UpdateAnimatedEmojiMessageClicked>),
    /// The parameters of animation search through getOption("animation_search_bot_username") bot have changed
    #[serde(rename(serialize = "updateAnimationSearchParameters", deserialize = "updateAnimationSearchParameters"))]
    AnimationSearchParameters(Box<crate::types::UpdateAnimationSearchParameters>),
    /// The styles supported for text composition have changed
    #[serde(rename(serialize = "updateTextCompositionStyles", deserialize = "updateTextCompositionStyles"))]
    TextCompositionStyles(Box<crate::types::UpdateTextCompositionStyles>),
    /// The list of suggested to the user actions has changed
    #[serde(rename(serialize = "updateSuggestedActions", deserialize = "updateSuggestedActions"))]
    SuggestedActions(Box<crate::types::UpdateSuggestedActions>),
    /// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user.
    /// Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
    #[serde(rename(serialize = "updateSpeedLimitNotification", deserialize = "updateSpeedLimitNotification"))]
    SpeedLimitNotification(Box<crate::types::UpdateSpeedLimitNotification>),
    /// The list of contacts that had birthdays recently or will have birthday soon has changed
    #[serde(rename(serialize = "updateContactCloseBirthdays", deserialize = "updateContactCloseBirthdays"))]
    ContactCloseBirthdays(Box<crate::types::UpdateContactCloseBirthdays>),
    /// Autosave settings for some type of chats were updated
    #[serde(rename(serialize = "updateAutosaveSettings", deserialize = "updateAutosaveSettings"))]
    AutosaveSettings(Box<crate::types::UpdateAutosaveSettings>),
    /// A business connection has changed; for bots only
    #[serde(rename(serialize = "updateBusinessConnection", deserialize = "updateBusinessConnection"))]
    BusinessConnection(Box<crate::types::UpdateBusinessConnection>),
    /// A new message was added to a business account; for bots only
    #[serde(rename(serialize = "updateNewBusinessMessage", deserialize = "updateNewBusinessMessage"))]
    NewBusinessMessage(Box<crate::types::UpdateNewBusinessMessage>),
    /// A message in a business account was edited; for bots only
    #[serde(rename(serialize = "updateBusinessMessageEdited", deserialize = "updateBusinessMessageEdited"))]
    BusinessMessageEdited(Box<crate::types::UpdateBusinessMessageEdited>),
    /// Messages in a business account were deleted; for bots only
    #[serde(rename(serialize = "updateBusinessMessagesDeleted", deserialize = "updateBusinessMessagesDeleted"))]
    BusinessMessagesDeleted(Box<crate::types::UpdateBusinessMessagesDeleted>),
    /// A new incoming inline query; for bots only
    #[serde(rename(serialize = "updateNewInlineQuery", deserialize = "updateNewInlineQuery"))]
    NewInlineQuery(Box<crate::types::UpdateNewInlineQuery>),
    /// The user has chosen a result of an inline query; for bots only
    #[serde(rename(serialize = "updateNewChosenInlineResult", deserialize = "updateNewChosenInlineResult"))]
    NewChosenInlineResult(Box<crate::types::UpdateNewChosenInlineResult>),
    /// A new incoming guest query; for bots only
    #[serde(rename(serialize = "updateNewGuestQuery", deserialize = "updateNewGuestQuery"))]
    NewGuestQuery(Box<crate::types::UpdateNewGuestQuery>),
    /// A new incoming callback query; for bots only
    #[serde(rename(serialize = "updateNewCallbackQuery", deserialize = "updateNewCallbackQuery"))]
    NewCallbackQuery(Box<crate::types::UpdateNewCallbackQuery>),
    /// A new incoming callback query from a message sent via a bot; for bots only
    #[serde(rename(serialize = "updateNewInlineCallbackQuery", deserialize = "updateNewInlineCallbackQuery"))]
    NewInlineCallbackQuery(Box<crate::types::UpdateNewInlineCallbackQuery>),
    /// A new incoming callback query from a business message; for bots only
    #[serde(rename(serialize = "updateNewBusinessCallbackQuery", deserialize = "updateNewBusinessCallbackQuery"))]
    NewBusinessCallbackQuery(Box<crate::types::UpdateNewBusinessCallbackQuery>),
    /// A new incoming shipping query; for bots only. Only for invoices with flexible price
    #[serde(rename(serialize = "updateNewShippingQuery", deserialize = "updateNewShippingQuery"))]
    NewShippingQuery(Box<crate::types::UpdateNewShippingQuery>),
    /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
    #[serde(rename(serialize = "updateNewPreCheckoutQuery", deserialize = "updateNewPreCheckoutQuery"))]
    NewPreCheckoutQuery(Box<crate::types::UpdateNewPreCheckoutQuery>),
    /// A new incoming event; for bots only
    #[serde(rename(serialize = "updateNewCustomEvent", deserialize = "updateNewCustomEvent"))]
    NewCustomEvent(Box<crate::types::UpdateNewCustomEvent>),
    /// A new incoming query; for bots only
    #[serde(rename(serialize = "updateNewCustomQuery", deserialize = "updateNewCustomQuery"))]
    NewCustomQuery(Box<crate::types::UpdateNewCustomQuery>),
    /// Subscription of a user to the bot was changed; for bots only
    #[serde(rename(serialize = "updateUserSubscription", deserialize = "updateUserSubscription"))]
    UserSubscription(Box<crate::types::UpdateUserSubscription>),
    /// A poll was updated; for bots only
    #[serde(rename(serialize = "updatePoll", deserialize = "updatePoll"))]
    Poll(Box<crate::types::UpdatePoll>),
    /// A user changed the answer to a poll; for bots only
    #[serde(rename(serialize = "updatePollAnswer", deserialize = "updatePollAnswer"))]
    PollAnswer(Box<crate::types::UpdatePollAnswer>),
    /// A bot that can be managed by the current bot was created or updated; for bots only
    #[serde(rename(serialize = "updateManagedBot", deserialize = "updateManagedBot"))]
    ManagedBot(Box<crate::types::UpdateManagedBot>),
    /// User rights changed in a chat; for bots only
    #[serde(rename(serialize = "updateChatMember", deserialize = "updateChatMember"))]
    ChatMember(Box<crate::types::UpdateChatMember>),
    /// A user sent a join request to a chat; for bots only
    #[serde(rename(serialize = "updateNewChatJoinRequest", deserialize = "updateNewChatJoinRequest"))]
    NewChatJoinRequest(Box<crate::types::UpdateNewChatJoinRequest>),
    /// A chat boost has changed; for bots only
    #[serde(rename(serialize = "updateChatBoost", deserialize = "updateChatBoost"))]
    ChatBoost(Box<crate::types::UpdateChatBoost>),
    /// User changed its reactions on a message with public reactions; for bots only
    #[serde(rename(serialize = "updateMessageReaction", deserialize = "updateMessageReaction"))]
    MessageReaction(Box<crate::types::UpdateMessageReaction>),
    /// Reactions added to a message with anonymous reactions have changed; for bots only
    #[serde(rename(serialize = "updateMessageReactions", deserialize = "updateMessageReactions"))]
    MessageReactions(Box<crate::types::UpdateMessageReactions>),
    /// Paid media were purchased by a user; for bots only
    #[serde(rename(serialize = "updatePaidMediaPurchased", deserialize = "updatePaidMediaPurchased"))]
    PaidMediaPurchased(Box<crate::types::UpdatePaidMediaPurchased>),
}

impl Update {
    /// Convenience constructor to create a [`Update::AuthorizationState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn authorization_state(val: crate::types::UpdateAuthorizationState) -> Self {
        Self::AuthorizationState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_message(val: crate::types::UpdateNewMessage) -> Self {
        Self::NewMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageSendAcknowledged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_send_acknowledged(val: crate::types::UpdateMessageSendAcknowledged) -> Self {
        Self::MessageSendAcknowledged(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageSendSucceeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_send_succeeded(val: crate::types::UpdateMessageSendSucceeded) -> Self {
        Self::MessageSendSucceeded(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageSendFailed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_send_failed(val: crate::types::UpdateMessageSendFailed) -> Self {
        Self::MessageSendFailed(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageContent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_content(val: crate::types::UpdateMessageContent) -> Self {
        Self::MessageContent(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageEphemeralContent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_ephemeral_content(val: crate::types::UpdateMessageEphemeralContent) -> Self {
        Self::MessageEphemeralContent(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_edited(val: crate::types::UpdateMessageEdited) -> Self {
        Self::MessageEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageIsPinned`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_is_pinned(val: crate::types::UpdateMessageIsPinned) -> Self {
        Self::MessageIsPinned(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageInteractionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_interaction_info(val: crate::types::UpdateMessageInteractionInfo) -> Self {
        Self::MessageInteractionInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageContentOpened`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_content_opened(val: crate::types::UpdateMessageContentOpened) -> Self {
        Self::MessageContentOpened(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageMentionRead`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_mention_read(val: crate::types::UpdateMessageMentionRead) -> Self {
        Self::MessageMentionRead(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageUnreadReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_unread_reactions(val: crate::types::UpdateMessageUnreadReactions) -> Self {
        Self::MessageUnreadReactions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageContainsUnreadPollVotes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_contains_unread_poll_votes(val: crate::types::UpdateMessageContainsUnreadPollVotes) -> Self {
        Self::MessageContainsUnreadPollVotes(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageFactCheck`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_fact_check(val: crate::types::UpdateMessageFactCheck) -> Self {
        Self::MessageFactCheck(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageSuggestedPostInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_info(val: crate::types::UpdateMessageSuggestedPostInfo) -> Self {
        Self::MessageSuggestedPostInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageLiveLocationViewed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_live_location_viewed(val: crate::types::UpdateMessageLiveLocationViewed) -> Self {
        Self::MessageLiveLocationViewed(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::VideoPublished`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_published(val: crate::types::UpdateVideoPublished) -> Self {
        Self::VideoPublished(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_chat(val: crate::types::UpdateNewChat) -> Self {
        Self::NewChat(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatTitle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_title(val: crate::types::UpdateChatTitle) -> Self {
        Self::ChatTitle(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatPhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_photo(val: crate::types::UpdateChatPhoto) -> Self {
        Self::ChatPhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatAccentColors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_accent_colors(val: crate::types::UpdateChatAccentColors) -> Self {
        Self::ChatAccentColors(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatPermissions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_permissions(val: crate::types::UpdateChatPermissions) -> Self {
        Self::ChatPermissions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatLastMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_last_message(val: crate::types::UpdateChatLastMessage) -> Self {
        Self::ChatLastMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatPosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_position(val: crate::types::UpdateChatPosition) -> Self {
        Self::ChatPosition(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatAddedToList`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_added_to_list(val: crate::types::UpdateChatAddedToList) -> Self {
        Self::ChatAddedToList(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatRemovedFromList`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_removed_from_list(val: crate::types::UpdateChatRemovedFromList) -> Self {
        Self::ChatRemovedFromList(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatReadInbox`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_read_inbox(val: crate::types::UpdateChatReadInbox) -> Self {
        Self::ChatReadInbox(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatReadOutbox`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_read_outbox(val: crate::types::UpdateChatReadOutbox) -> Self {
        Self::ChatReadOutbox(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatActionBar`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_action_bar(val: crate::types::UpdateChatActionBar) -> Self {
        Self::ChatActionBar(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatBusinessBotManageBar`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_business_bot_manage_bar(val: crate::types::UpdateChatBusinessBotManageBar) -> Self {
        Self::ChatBusinessBotManageBar(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatAvailableReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_available_reactions(val: crate::types::UpdateChatAvailableReactions) -> Self {
        Self::ChatAvailableReactions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatDraftMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_draft_message(val: crate::types::UpdateChatDraftMessage) -> Self {
        Self::ChatDraftMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatEmojiStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_emoji_status(val: crate::types::UpdateChatEmojiStatus) -> Self {
        Self::ChatEmojiStatus(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatMessageSender`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_message_sender(val: crate::types::UpdateChatMessageSender) -> Self {
        Self::ChatMessageSender(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatMessageAutoDeleteTime`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_message_auto_delete_time(val: crate::types::UpdateChatMessageAutoDeleteTime) -> Self {
        Self::ChatMessageAutoDeleteTime(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_notification_settings(val: crate::types::UpdateChatNotificationSettings) -> Self {
        Self::ChatNotificationSettings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatPendingJoinRequests`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_pending_join_requests(val: crate::types::UpdateChatPendingJoinRequests) -> Self {
        Self::ChatPendingJoinRequests(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatReplyMarkup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_reply_markup(val: crate::types::UpdateChatReplyMarkup) -> Self {
        Self::ChatReplyMarkup(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_background(val: crate::types::UpdateChatBackground) -> Self {
        Self::ChatBackground(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_theme(val: crate::types::UpdateChatTheme) -> Self {
        Self::ChatTheme(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatUnreadMentionCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_unread_mention_count(val: crate::types::UpdateChatUnreadMentionCount) -> Self {
        Self::ChatUnreadMentionCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatUnreadReactionCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_unread_reaction_count(val: crate::types::UpdateChatUnreadReactionCount) -> Self {
        Self::ChatUnreadReactionCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatUnreadPollVoteCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_unread_poll_vote_count(val: crate::types::UpdateChatUnreadPollVoteCount) -> Self {
        Self::ChatUnreadPollVoteCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatVideoChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_video_chat(val: crate::types::UpdateChatVideoChat) -> Self {
        Self::ChatVideoChat(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatDefaultDisableNotification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_default_disable_notification(val: crate::types::UpdateChatDefaultDisableNotification) -> Self {
        Self::ChatDefaultDisableNotification(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatHasProtectedContent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_has_protected_content(val: crate::types::UpdateChatHasProtectedContent) -> Self {
        Self::ChatHasProtectedContent(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatIsTranslatable`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_is_translatable(val: crate::types::UpdateChatIsTranslatable) -> Self {
        Self::ChatIsTranslatable(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatIsMarkedAsUnread`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_is_marked_as_unread(val: crate::types::UpdateChatIsMarkedAsUnread) -> Self {
        Self::ChatIsMarkedAsUnread(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatViewAsTopics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_view_as_topics(val: crate::types::UpdateChatViewAsTopics) -> Self {
        Self::ChatViewAsTopics(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatBlockList`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_block_list(val: crate::types::UpdateChatBlockList) -> Self {
        Self::ChatBlockList(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatHasScheduledMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_has_scheduled_messages(val: crate::types::UpdateChatHasScheduledMessages) -> Self {
        Self::ChatHasScheduledMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatHasWelcomeMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_has_welcome_messages(val: crate::types::UpdateChatHasWelcomeMessages) -> Self {
        Self::ChatHasWelcomeMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatFolders`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_folders(val: crate::types::UpdateChatFolders) -> Self {
        Self::ChatFolders(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatOnlineMemberCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_online_member_count(val: crate::types::UpdateChatOnlineMemberCount) -> Self {
        Self::ChatOnlineMemberCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SavedMessagesTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_topic(val: crate::types::UpdateSavedMessagesTopic) -> Self {
        Self::SavedMessagesTopic(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SavedMessagesTopicCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_topic_count(val: crate::types::UpdateSavedMessagesTopicCount) -> Self {
        Self::SavedMessagesTopicCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DirectMessagesChatTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn direct_messages_chat_topic(val: crate::types::UpdateDirectMessagesChatTopic) -> Self {
        Self::DirectMessagesChatTopic(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::TopicMessageCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn topic_message_count(val: crate::types::UpdateTopicMessageCount) -> Self {
        Self::TopicMessageCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::QuickReplyShortcut`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_shortcut(val: crate::types::UpdateQuickReplyShortcut) -> Self {
        Self::QuickReplyShortcut(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::QuickReplyShortcutDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_shortcut_deleted(val: crate::types::UpdateQuickReplyShortcutDeleted) -> Self {
        Self::QuickReplyShortcutDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::QuickReplyShortcuts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_shortcuts(val: crate::types::UpdateQuickReplyShortcuts) -> Self {
        Self::QuickReplyShortcuts(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::QuickReplyShortcutMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_shortcut_messages(val: crate::types::UpdateQuickReplyShortcutMessages) -> Self {
        Self::QuickReplyShortcutMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatWelcomeMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_welcome_messages(val: crate::types::UpdateChatWelcomeMessages) -> Self {
        Self::ChatWelcomeMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ForumTopicInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topic_info(val: crate::types::UpdateForumTopicInfo) -> Self {
        Self::ForumTopicInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ForumTopic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forum_topic(val: crate::types::UpdateForumTopic) -> Self {
        Self::ForumTopic(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ScopeNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn scope_notification_settings(val: crate::types::UpdateScopeNotificationSettings) -> Self {
        Self::ScopeNotificationSettings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ReactionNotificationSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn reaction_notification_settings(val: crate::types::UpdateReactionNotificationSettings) -> Self {
        Self::ReactionNotificationSettings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Notification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification(val: crate::types::UpdateNotification) -> Self {
        Self::Notification(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NotificationGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn notification_group(val: crate::types::UpdateNotificationGroup) -> Self {
        Self::NotificationGroup(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ActiveNotifications`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn active_notifications(val: crate::types::UpdateActiveNotifications) -> Self {
        Self::ActiveNotifications(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::HavePendingNotifications`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn have_pending_notifications(val: crate::types::UpdateHavePendingNotifications) -> Self {
        Self::HavePendingNotifications(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DeleteMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn delete_messages(val: crate::types::UpdateDeleteMessages) -> Self {
        Self::DeleteMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatAction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_action(val: crate::types::UpdateChatAction) -> Self {
        Self::ChatAction(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::PendingMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending_message(val: crate::types::UpdatePendingMessage) -> Self {
        Self::PendingMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StopMessageDraft`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stop_message_draft(val: crate::types::UpdateStopMessageDraft) -> Self {
        Self::StopMessageDraft(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Community`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community(val: crate::types::UpdateCommunity) -> Self {
        Self::Community(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UserStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_status(val: crate::types::UpdateUserStatus) -> Self {
        Self::UserStatus(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::UpdateUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::BasicGroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn basic_group(val: crate::types::UpdateBasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Supergroup`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup(val: crate::types::UpdateSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SecretChat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn secret_chat(val: crate::types::UpdateSecretChat) -> Self {
        Self::SecretChat(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UserFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_full_info(val: crate::types::UpdateUserFullInfo) -> Self {
        Self::UserFullInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::BasicGroupFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn basic_group_full_info(val: crate::types::UpdateBasicGroupFullInfo) -> Self {
        Self::BasicGroupFullInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SupergroupFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn supergroup_full_info(val: crate::types::UpdateSupergroupFullInfo) -> Self {
        Self::SupergroupFullInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::CommunityFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community_full_info(val: crate::types::UpdateCommunityFullInfo) -> Self {
        Self::CommunityFullInfo(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ServiceNotification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn service_notification(val: crate::types::UpdateServiceNotification) -> Self {
        Self::ServiceNotification(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewOauthRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_oauth_request(val: crate::types::UpdateNewOauthRequest) -> Self {
        Self::NewOauthRequest(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::File`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file(val: crate::types::UpdateFile) -> Self {
        Self::File(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileGenerationStart`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_generation_start(val: crate::types::UpdateFileGenerationStart) -> Self {
        Self::FileGenerationStart(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileGenerationStop`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_generation_stop(val: crate::types::UpdateFileGenerationStop) -> Self {
        Self::FileGenerationStop(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileDownloads`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_downloads(val: crate::types::UpdateFileDownloads) -> Self {
        Self::FileDownloads(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileAddedToDownloads`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_added_to_downloads(val: crate::types::UpdateFileAddedToDownloads) -> Self {
        Self::FileAddedToDownloads(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileDownload`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_download(val: crate::types::UpdateFileDownload) -> Self {
        Self::FileDownload(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FileRemovedFromDownloads`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file_removed_from_downloads(val: crate::types::UpdateFileRemovedFromDownloads) -> Self {
        Self::FileRemovedFromDownloads(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ApplicationVerificationRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn application_verification_required(val: crate::types::UpdateApplicationVerificationRequired) -> Self {
        Self::ApplicationVerificationRequired(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ApplicationRecaptchaVerificationRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn application_recaptcha_verification_required(val: crate::types::UpdateApplicationRecaptchaVerificationRequired) -> Self {
        Self::ApplicationRecaptchaVerificationRequired(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Call`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn call(val: crate::types::UpdateCall) -> Self {
        Self::Call(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call(val: crate::types::UpdateGroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallParticipant`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_participant(val: crate::types::UpdateGroupCallParticipant) -> Self {
        Self::GroupCallParticipant(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallParticipants`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_participants(val: crate::types::UpdateGroupCallParticipants) -> Self {
        Self::GroupCallParticipants(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallVerificationState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_verification_state(val: crate::types::UpdateGroupCallVerificationState) -> Self {
        Self::GroupCallVerificationState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewGroupCallMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_group_call_message(val: crate::types::UpdateNewGroupCallMessage) -> Self {
        Self::NewGroupCallMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewGroupCallPaidReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_group_call_paid_reaction(val: crate::types::UpdateNewGroupCallPaidReaction) -> Self {
        Self::NewGroupCallPaidReaction(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallMessageSendFailed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_message_send_failed(val: crate::types::UpdateGroupCallMessageSendFailed) -> Self {
        Self::GroupCallMessageSendFailed(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallMessagesDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_messages_deleted(val: crate::types::UpdateGroupCallMessagesDeleted) -> Self {
        Self::GroupCallMessagesDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::LiveStoryTopDonors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_story_top_donors(val: crate::types::UpdateLiveStoryTopDonors) -> Self {
        Self::LiveStoryTopDonors(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewCallSignalingData`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_call_signaling_data(val: crate::types::UpdateNewCallSignalingData) -> Self {
        Self::NewCallSignalingData(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GiftAuctionState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift_auction_state(val: crate::types::UpdateGiftAuctionState) -> Self {
        Self::GiftAuctionState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ActiveGiftAuctions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn active_gift_auctions(val: crate::types::UpdateActiveGiftAuctions) -> Self {
        Self::ActiveGiftAuctions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UserPrivacySettingRules`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_privacy_setting_rules(val: crate::types::UpdateUserPrivacySettingRules) -> Self {
        Self::UserPrivacySettingRules(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UnreadMessageCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unread_message_count(val: crate::types::UpdateUnreadMessageCount) -> Self {
        Self::UnreadMessageCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UnreadChatCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unread_chat_count(val: crate::types::UpdateUnreadChatCount) -> Self {
        Self::UnreadChatCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatJoinResult`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_join_result(val: crate::types::UpdateChatJoinResult) -> Self {
        Self::ChatJoinResult(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::UpdateStory) -> Self {
        Self::Story(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StoryDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_deleted(val: crate::types::UpdateStoryDeleted) -> Self {
        Self::StoryDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StoryPostSucceeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_post_succeeded(val: crate::types::UpdateStoryPostSucceeded) -> Self {
        Self::StoryPostSucceeded(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StoryPostFailed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_post_failed(val: crate::types::UpdateStoryPostFailed) -> Self {
        Self::StoryPostFailed(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatActiveStories`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_active_stories(val: crate::types::UpdateChatActiveStories) -> Self {
        Self::ChatActiveStories(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StoryListChatCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_list_chat_count(val: crate::types::UpdateStoryListChatCount) -> Self {
        Self::StoryListChatCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StoryStealthMode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_stealth_mode(val: crate::types::UpdateStoryStealthMode) -> Self {
        Self::StoryStealthMode(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::TrustedMiniAppBots`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn trusted_mini_app_bots(val: crate::types::UpdateTrustedMiniAppBots) -> Self {
        Self::TrustedMiniAppBots(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Option`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn option(val: crate::types::UpdateOption) -> Self {
        Self::Option(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StickerSet`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker_set(val: crate::types::UpdateStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::InstalledStickerSets`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn installed_sticker_sets(val: crate::types::UpdateInstalledStickerSets) -> Self {
        Self::InstalledStickerSets(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::TrendingStickerSets`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn trending_sticker_sets(val: crate::types::UpdateTrendingStickerSets) -> Self {
        Self::TrendingStickerSets(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::RecentStickers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recent_stickers(val: crate::types::UpdateRecentStickers) -> Self {
        Self::RecentStickers(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FavoriteStickers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn favorite_stickers(val: crate::types::UpdateFavoriteStickers) -> Self {
        Self::FavoriteStickers(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SavedAnimations`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_animations(val: crate::types::UpdateSavedAnimations) -> Self {
        Self::SavedAnimations(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SavedNotificationSounds`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_notification_sounds(val: crate::types::UpdateSavedNotificationSounds) -> Self {
        Self::SavedNotificationSounds(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DefaultBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn default_background(val: crate::types::UpdateDefaultBackground) -> Self {
        Self::DefaultBackground(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::EmojiChatThemes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_chat_themes(val: crate::types::UpdateEmojiChatThemes) -> Self {
        Self::EmojiChatThemes(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AccentColors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn accent_colors(val: crate::types::UpdateAccentColors) -> Self {
        Self::AccentColors(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ProfileAccentColors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn profile_accent_colors(val: crate::types::UpdateProfileAccentColors) -> Self {
        Self::ProfileAccentColors(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::WebBrowserSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_browser_settings(val: crate::types::UpdateWebBrowserSettings) -> Self {
        Self::WebBrowserSettings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::LanguagePackStrings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn language_pack_strings(val: crate::types::UpdateLanguagePackStrings) -> Self {
        Self::LanguagePackStrings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ConnectionState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn connection_state(val: crate::types::UpdateConnectionState) -> Self {
        Self::ConnectionState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::FreezeState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn freeze_state(val: crate::types::UpdateFreezeState) -> Self {
        Self::FreezeState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AgeVerificationParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn age_verification_parameters(val: crate::types::UpdateAgeVerificationParameters) -> Self {
        Self::AgeVerificationParameters(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::TermsOfService`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn terms_of_service(val: crate::types::UpdateTermsOfService) -> Self {
        Self::TermsOfService(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UnconfirmedSession`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unconfirmed_session(val: crate::types::UpdateUnconfirmedSession) -> Self {
        Self::UnconfirmedSession(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AttachmentMenuBots`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn attachment_menu_bots(val: crate::types::UpdateAttachmentMenuBots) -> Self {
        Self::AttachmentMenuBots(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::WebAppMessageSent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn web_app_message_sent(val: crate::types::UpdateWebAppMessageSent) -> Self {
        Self::WebAppMessageSent(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ActiveEmojiReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn active_emoji_reactions(val: crate::types::UpdateActiveEmojiReactions) -> Self {
        Self::ActiveEmojiReactions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AvailableMessageEffects`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available_message_effects(val: crate::types::UpdateAvailableMessageEffects) -> Self {
        Self::AvailableMessageEffects(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DefaultReactionType`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn default_reaction_type(val: crate::types::UpdateDefaultReactionType) -> Self {
        Self::DefaultReactionType(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DefaultPaidReactionType`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn default_paid_reaction_type(val: crate::types::UpdateDefaultPaidReactionType) -> Self {
        Self::DefaultPaidReactionType(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SavedMessagesTags`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_tags(val: crate::types::UpdateSavedMessagesTags) -> Self {
        Self::SavedMessagesTags(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ActiveLiveLocationMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn active_live_location_messages(val: crate::types::UpdateActiveLiveLocationMessages) -> Self {
        Self::ActiveLiveLocationMessages(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::OwnedStarCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn owned_star_count(val: crate::types::UpdateOwnedStarCount) -> Self {
        Self::OwnedStarCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::OwnedGramCount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn owned_gram_count(val: crate::types::UpdateOwnedGramCount) -> Self {
        Self::OwnedGramCount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatRevenueAmount`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_revenue_amount(val: crate::types::UpdateChatRevenueAmount) -> Self {
        Self::ChatRevenueAmount(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StarRevenueStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn star_revenue_status(val: crate::types::UpdateStarRevenueStatus) -> Self {
        Self::StarRevenueStatus(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GramRevenueStatus`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gram_revenue_status(val: crate::types::UpdateGramRevenueStatus) -> Self {
        Self::GramRevenueStatus(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SpeechRecognitionTrial`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn speech_recognition_trial(val: crate::types::UpdateSpeechRecognitionTrial) -> Self {
        Self::SpeechRecognitionTrial(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::GroupCallMessageLevels`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn group_call_message_levels(val: crate::types::UpdateGroupCallMessageLevels) -> Self {
        Self::GroupCallMessageLevels(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::DiceEmojis`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn dice_emojis(val: crate::types::UpdateDiceEmojis) -> Self {
        Self::DiceEmojis(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::StakeDiceState`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stake_dice_state(val: crate::types::UpdateStakeDiceState) -> Self {
        Self::StakeDiceState(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AnimatedEmojiMessageClicked`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animated_emoji_message_clicked(val: crate::types::UpdateAnimatedEmojiMessageClicked) -> Self {
        Self::AnimatedEmojiMessageClicked(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AnimationSearchParameters`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation_search_parameters(val: crate::types::UpdateAnimationSearchParameters) -> Self {
        Self::AnimationSearchParameters(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::TextCompositionStyles`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_composition_styles(val: crate::types::UpdateTextCompositionStyles) -> Self {
        Self::TextCompositionStyles(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SuggestedActions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_actions(val: crate::types::UpdateSuggestedActions) -> Self {
        Self::SuggestedActions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::SpeedLimitNotification`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn speed_limit_notification(val: crate::types::UpdateSpeedLimitNotification) -> Self {
        Self::SpeedLimitNotification(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ContactCloseBirthdays`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact_close_birthdays(val: crate::types::UpdateContactCloseBirthdays) -> Self {
        Self::ContactCloseBirthdays(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::AutosaveSettings`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn autosave_settings(val: crate::types::UpdateAutosaveSettings) -> Self {
        Self::AutosaveSettings(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::BusinessConnection`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_connection(val: crate::types::UpdateBusinessConnection) -> Self {
        Self::BusinessConnection(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewBusinessMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_business_message(val: crate::types::UpdateNewBusinessMessage) -> Self {
        Self::NewBusinessMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::BusinessMessageEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_message_edited(val: crate::types::UpdateBusinessMessageEdited) -> Self {
        Self::BusinessMessageEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::BusinessMessagesDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn business_messages_deleted(val: crate::types::UpdateBusinessMessagesDeleted) -> Self {
        Self::BusinessMessagesDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewInlineQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_inline_query(val: crate::types::UpdateNewInlineQuery) -> Self {
        Self::NewInlineQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewChosenInlineResult`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_chosen_inline_result(val: crate::types::UpdateNewChosenInlineResult) -> Self {
        Self::NewChosenInlineResult(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewGuestQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_guest_query(val: crate::types::UpdateNewGuestQuery) -> Self {
        Self::NewGuestQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewCallbackQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_callback_query(val: crate::types::UpdateNewCallbackQuery) -> Self {
        Self::NewCallbackQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewInlineCallbackQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_inline_callback_query(val: crate::types::UpdateNewInlineCallbackQuery) -> Self {
        Self::NewInlineCallbackQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewBusinessCallbackQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_business_callback_query(val: crate::types::UpdateNewBusinessCallbackQuery) -> Self {
        Self::NewBusinessCallbackQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewShippingQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_shipping_query(val: crate::types::UpdateNewShippingQuery) -> Self {
        Self::NewShippingQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewPreCheckoutQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_pre_checkout_query(val: crate::types::UpdateNewPreCheckoutQuery) -> Self {
        Self::NewPreCheckoutQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewCustomEvent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_custom_event(val: crate::types::UpdateNewCustomEvent) -> Self {
        Self::NewCustomEvent(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewCustomQuery`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_custom_query(val: crate::types::UpdateNewCustomQuery) -> Self {
        Self::NewCustomQuery(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::UserSubscription`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_subscription(val: crate::types::UpdateUserSubscription) -> Self {
        Self::UserSubscription(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::Poll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll(val: crate::types::UpdatePoll) -> Self {
        Self::Poll(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::PollAnswer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_answer(val: crate::types::UpdatePollAnswer) -> Self {
        Self::PollAnswer(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ManagedBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn managed_bot(val: crate::types::UpdateManagedBot) -> Self {
        Self::ManagedBot(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_member(val: crate::types::UpdateChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::NewChatJoinRequest`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn new_chat_join_request(val: crate::types::UpdateNewChatJoinRequest) -> Self {
        Self::NewChatJoinRequest(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::ChatBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_boost(val: crate::types::UpdateChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_reaction(val: crate::types::UpdateMessageReaction) -> Self {
        Self::MessageReaction(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::MessageReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_reactions(val: crate::types::UpdateMessageReactions) -> Self {
        Self::MessageReactions(Box::new(val))
    }

    /// Convenience constructor to create a [`Update::PaidMediaPurchased`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_media_purchased(val: crate::types::UpdatePaidMediaPurchased) -> Self {
        Self::PaidMediaPurchased(Box::new(val))
    }

}

/// Converts a [`crate::types::UpdateAuthorizationState`] into [`Update`].
impl From<crate::types::UpdateAuthorizationState> for Update {
    fn from(val: crate::types::UpdateAuthorizationState) -> Self {
        Self::AuthorizationState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewMessage`] into [`Update`].
impl From<crate::types::UpdateNewMessage> for Update {
    fn from(val: crate::types::UpdateNewMessage) -> Self {
        Self::NewMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageSendAcknowledged`] into [`Update`].
impl From<crate::types::UpdateMessageSendAcknowledged> for Update {
    fn from(val: crate::types::UpdateMessageSendAcknowledged) -> Self {
        Self::MessageSendAcknowledged(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageSendSucceeded`] into [`Update`].
impl From<crate::types::UpdateMessageSendSucceeded> for Update {
    fn from(val: crate::types::UpdateMessageSendSucceeded) -> Self {
        Self::MessageSendSucceeded(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageSendFailed`] into [`Update`].
impl From<crate::types::UpdateMessageSendFailed> for Update {
    fn from(val: crate::types::UpdateMessageSendFailed) -> Self {
        Self::MessageSendFailed(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageContent`] into [`Update`].
impl From<crate::types::UpdateMessageContent> for Update {
    fn from(val: crate::types::UpdateMessageContent) -> Self {
        Self::MessageContent(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageEphemeralContent`] into [`Update`].
impl From<crate::types::UpdateMessageEphemeralContent> for Update {
    fn from(val: crate::types::UpdateMessageEphemeralContent) -> Self {
        Self::MessageEphemeralContent(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageEdited`] into [`Update`].
impl From<crate::types::UpdateMessageEdited> for Update {
    fn from(val: crate::types::UpdateMessageEdited) -> Self {
        Self::MessageEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageIsPinned`] into [`Update`].
impl From<crate::types::UpdateMessageIsPinned> for Update {
    fn from(val: crate::types::UpdateMessageIsPinned) -> Self {
        Self::MessageIsPinned(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageInteractionInfo`] into [`Update`].
impl From<crate::types::UpdateMessageInteractionInfo> for Update {
    fn from(val: crate::types::UpdateMessageInteractionInfo) -> Self {
        Self::MessageInteractionInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageContentOpened`] into [`Update`].
impl From<crate::types::UpdateMessageContentOpened> for Update {
    fn from(val: crate::types::UpdateMessageContentOpened) -> Self {
        Self::MessageContentOpened(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageMentionRead`] into [`Update`].
impl From<crate::types::UpdateMessageMentionRead> for Update {
    fn from(val: crate::types::UpdateMessageMentionRead) -> Self {
        Self::MessageMentionRead(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageUnreadReactions`] into [`Update`].
impl From<crate::types::UpdateMessageUnreadReactions> for Update {
    fn from(val: crate::types::UpdateMessageUnreadReactions) -> Self {
        Self::MessageUnreadReactions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageContainsUnreadPollVotes`] into [`Update`].
impl From<crate::types::UpdateMessageContainsUnreadPollVotes> for Update {
    fn from(val: crate::types::UpdateMessageContainsUnreadPollVotes) -> Self {
        Self::MessageContainsUnreadPollVotes(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageFactCheck`] into [`Update`].
impl From<crate::types::UpdateMessageFactCheck> for Update {
    fn from(val: crate::types::UpdateMessageFactCheck) -> Self {
        Self::MessageFactCheck(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageSuggestedPostInfo`] into [`Update`].
impl From<crate::types::UpdateMessageSuggestedPostInfo> for Update {
    fn from(val: crate::types::UpdateMessageSuggestedPostInfo) -> Self {
        Self::MessageSuggestedPostInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageLiveLocationViewed`] into [`Update`].
impl From<crate::types::UpdateMessageLiveLocationViewed> for Update {
    fn from(val: crate::types::UpdateMessageLiveLocationViewed) -> Self {
        Self::MessageLiveLocationViewed(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateVideoPublished`] into [`Update`].
impl From<crate::types::UpdateVideoPublished> for Update {
    fn from(val: crate::types::UpdateVideoPublished) -> Self {
        Self::VideoPublished(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewChat`] into [`Update`].
impl From<crate::types::UpdateNewChat> for Update {
    fn from(val: crate::types::UpdateNewChat) -> Self {
        Self::NewChat(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatTitle`] into [`Update`].
impl From<crate::types::UpdateChatTitle> for Update {
    fn from(val: crate::types::UpdateChatTitle) -> Self {
        Self::ChatTitle(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatPhoto`] into [`Update`].
impl From<crate::types::UpdateChatPhoto> for Update {
    fn from(val: crate::types::UpdateChatPhoto) -> Self {
        Self::ChatPhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatAccentColors`] into [`Update`].
impl From<crate::types::UpdateChatAccentColors> for Update {
    fn from(val: crate::types::UpdateChatAccentColors) -> Self {
        Self::ChatAccentColors(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatPermissions`] into [`Update`].
impl From<crate::types::UpdateChatPermissions> for Update {
    fn from(val: crate::types::UpdateChatPermissions) -> Self {
        Self::ChatPermissions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatLastMessage`] into [`Update`].
impl From<crate::types::UpdateChatLastMessage> for Update {
    fn from(val: crate::types::UpdateChatLastMessage) -> Self {
        Self::ChatLastMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatPosition`] into [`Update`].
impl From<crate::types::UpdateChatPosition> for Update {
    fn from(val: crate::types::UpdateChatPosition) -> Self {
        Self::ChatPosition(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatAddedToList`] into [`Update`].
impl From<crate::types::UpdateChatAddedToList> for Update {
    fn from(val: crate::types::UpdateChatAddedToList) -> Self {
        Self::ChatAddedToList(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatRemovedFromList`] into [`Update`].
impl From<crate::types::UpdateChatRemovedFromList> for Update {
    fn from(val: crate::types::UpdateChatRemovedFromList) -> Self {
        Self::ChatRemovedFromList(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatReadInbox`] into [`Update`].
impl From<crate::types::UpdateChatReadInbox> for Update {
    fn from(val: crate::types::UpdateChatReadInbox) -> Self {
        Self::ChatReadInbox(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatReadOutbox`] into [`Update`].
impl From<crate::types::UpdateChatReadOutbox> for Update {
    fn from(val: crate::types::UpdateChatReadOutbox) -> Self {
        Self::ChatReadOutbox(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatActionBar`] into [`Update`].
impl From<crate::types::UpdateChatActionBar> for Update {
    fn from(val: crate::types::UpdateChatActionBar) -> Self {
        Self::ChatActionBar(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatBusinessBotManageBar`] into [`Update`].
impl From<crate::types::UpdateChatBusinessBotManageBar> for Update {
    fn from(val: crate::types::UpdateChatBusinessBotManageBar) -> Self {
        Self::ChatBusinessBotManageBar(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatAvailableReactions`] into [`Update`].
impl From<crate::types::UpdateChatAvailableReactions> for Update {
    fn from(val: crate::types::UpdateChatAvailableReactions) -> Self {
        Self::ChatAvailableReactions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatDraftMessage`] into [`Update`].
impl From<crate::types::UpdateChatDraftMessage> for Update {
    fn from(val: crate::types::UpdateChatDraftMessage) -> Self {
        Self::ChatDraftMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatEmojiStatus`] into [`Update`].
impl From<crate::types::UpdateChatEmojiStatus> for Update {
    fn from(val: crate::types::UpdateChatEmojiStatus) -> Self {
        Self::ChatEmojiStatus(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatMessageSender`] into [`Update`].
impl From<crate::types::UpdateChatMessageSender> for Update {
    fn from(val: crate::types::UpdateChatMessageSender) -> Self {
        Self::ChatMessageSender(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatMessageAutoDeleteTime`] into [`Update`].
impl From<crate::types::UpdateChatMessageAutoDeleteTime> for Update {
    fn from(val: crate::types::UpdateChatMessageAutoDeleteTime) -> Self {
        Self::ChatMessageAutoDeleteTime(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatNotificationSettings`] into [`Update`].
impl From<crate::types::UpdateChatNotificationSettings> for Update {
    fn from(val: crate::types::UpdateChatNotificationSettings) -> Self {
        Self::ChatNotificationSettings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatPendingJoinRequests`] into [`Update`].
impl From<crate::types::UpdateChatPendingJoinRequests> for Update {
    fn from(val: crate::types::UpdateChatPendingJoinRequests) -> Self {
        Self::ChatPendingJoinRequests(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatReplyMarkup`] into [`Update`].
impl From<crate::types::UpdateChatReplyMarkup> for Update {
    fn from(val: crate::types::UpdateChatReplyMarkup) -> Self {
        Self::ChatReplyMarkup(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatBackground`] into [`Update`].
impl From<crate::types::UpdateChatBackground> for Update {
    fn from(val: crate::types::UpdateChatBackground) -> Self {
        Self::ChatBackground(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatTheme`] into [`Update`].
impl From<crate::types::UpdateChatTheme> for Update {
    fn from(val: crate::types::UpdateChatTheme) -> Self {
        Self::ChatTheme(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatUnreadMentionCount`] into [`Update`].
impl From<crate::types::UpdateChatUnreadMentionCount> for Update {
    fn from(val: crate::types::UpdateChatUnreadMentionCount) -> Self {
        Self::ChatUnreadMentionCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatUnreadReactionCount`] into [`Update`].
impl From<crate::types::UpdateChatUnreadReactionCount> for Update {
    fn from(val: crate::types::UpdateChatUnreadReactionCount) -> Self {
        Self::ChatUnreadReactionCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatUnreadPollVoteCount`] into [`Update`].
impl From<crate::types::UpdateChatUnreadPollVoteCount> for Update {
    fn from(val: crate::types::UpdateChatUnreadPollVoteCount) -> Self {
        Self::ChatUnreadPollVoteCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatVideoChat`] into [`Update`].
impl From<crate::types::UpdateChatVideoChat> for Update {
    fn from(val: crate::types::UpdateChatVideoChat) -> Self {
        Self::ChatVideoChat(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatDefaultDisableNotification`] into [`Update`].
impl From<crate::types::UpdateChatDefaultDisableNotification> for Update {
    fn from(val: crate::types::UpdateChatDefaultDisableNotification) -> Self {
        Self::ChatDefaultDisableNotification(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatHasProtectedContent`] into [`Update`].
impl From<crate::types::UpdateChatHasProtectedContent> for Update {
    fn from(val: crate::types::UpdateChatHasProtectedContent) -> Self {
        Self::ChatHasProtectedContent(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatIsTranslatable`] into [`Update`].
impl From<crate::types::UpdateChatIsTranslatable> for Update {
    fn from(val: crate::types::UpdateChatIsTranslatable) -> Self {
        Self::ChatIsTranslatable(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatIsMarkedAsUnread`] into [`Update`].
impl From<crate::types::UpdateChatIsMarkedAsUnread> for Update {
    fn from(val: crate::types::UpdateChatIsMarkedAsUnread) -> Self {
        Self::ChatIsMarkedAsUnread(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatViewAsTopics`] into [`Update`].
impl From<crate::types::UpdateChatViewAsTopics> for Update {
    fn from(val: crate::types::UpdateChatViewAsTopics) -> Self {
        Self::ChatViewAsTopics(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatBlockList`] into [`Update`].
impl From<crate::types::UpdateChatBlockList> for Update {
    fn from(val: crate::types::UpdateChatBlockList) -> Self {
        Self::ChatBlockList(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatHasScheduledMessages`] into [`Update`].
impl From<crate::types::UpdateChatHasScheduledMessages> for Update {
    fn from(val: crate::types::UpdateChatHasScheduledMessages) -> Self {
        Self::ChatHasScheduledMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatHasWelcomeMessages`] into [`Update`].
impl From<crate::types::UpdateChatHasWelcomeMessages> for Update {
    fn from(val: crate::types::UpdateChatHasWelcomeMessages) -> Self {
        Self::ChatHasWelcomeMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatFolders`] into [`Update`].
impl From<crate::types::UpdateChatFolders> for Update {
    fn from(val: crate::types::UpdateChatFolders) -> Self {
        Self::ChatFolders(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatOnlineMemberCount`] into [`Update`].
impl From<crate::types::UpdateChatOnlineMemberCount> for Update {
    fn from(val: crate::types::UpdateChatOnlineMemberCount) -> Self {
        Self::ChatOnlineMemberCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSavedMessagesTopic`] into [`Update`].
impl From<crate::types::UpdateSavedMessagesTopic> for Update {
    fn from(val: crate::types::UpdateSavedMessagesTopic) -> Self {
        Self::SavedMessagesTopic(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSavedMessagesTopicCount`] into [`Update`].
impl From<crate::types::UpdateSavedMessagesTopicCount> for Update {
    fn from(val: crate::types::UpdateSavedMessagesTopicCount) -> Self {
        Self::SavedMessagesTopicCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDirectMessagesChatTopic`] into [`Update`].
impl From<crate::types::UpdateDirectMessagesChatTopic> for Update {
    fn from(val: crate::types::UpdateDirectMessagesChatTopic) -> Self {
        Self::DirectMessagesChatTopic(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateTopicMessageCount`] into [`Update`].
impl From<crate::types::UpdateTopicMessageCount> for Update {
    fn from(val: crate::types::UpdateTopicMessageCount) -> Self {
        Self::TopicMessageCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateQuickReplyShortcut`] into [`Update`].
impl From<crate::types::UpdateQuickReplyShortcut> for Update {
    fn from(val: crate::types::UpdateQuickReplyShortcut) -> Self {
        Self::QuickReplyShortcut(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateQuickReplyShortcutDeleted`] into [`Update`].
impl From<crate::types::UpdateQuickReplyShortcutDeleted> for Update {
    fn from(val: crate::types::UpdateQuickReplyShortcutDeleted) -> Self {
        Self::QuickReplyShortcutDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateQuickReplyShortcuts`] into [`Update`].
impl From<crate::types::UpdateQuickReplyShortcuts> for Update {
    fn from(val: crate::types::UpdateQuickReplyShortcuts) -> Self {
        Self::QuickReplyShortcuts(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateQuickReplyShortcutMessages`] into [`Update`].
impl From<crate::types::UpdateQuickReplyShortcutMessages> for Update {
    fn from(val: crate::types::UpdateQuickReplyShortcutMessages) -> Self {
        Self::QuickReplyShortcutMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatWelcomeMessages`] into [`Update`].
impl From<crate::types::UpdateChatWelcomeMessages> for Update {
    fn from(val: crate::types::UpdateChatWelcomeMessages) -> Self {
        Self::ChatWelcomeMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateForumTopicInfo`] into [`Update`].
impl From<crate::types::UpdateForumTopicInfo> for Update {
    fn from(val: crate::types::UpdateForumTopicInfo) -> Self {
        Self::ForumTopicInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateForumTopic`] into [`Update`].
impl From<crate::types::UpdateForumTopic> for Update {
    fn from(val: crate::types::UpdateForumTopic) -> Self {
        Self::ForumTopic(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateScopeNotificationSettings`] into [`Update`].
impl From<crate::types::UpdateScopeNotificationSettings> for Update {
    fn from(val: crate::types::UpdateScopeNotificationSettings) -> Self {
        Self::ScopeNotificationSettings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateReactionNotificationSettings`] into [`Update`].
impl From<crate::types::UpdateReactionNotificationSettings> for Update {
    fn from(val: crate::types::UpdateReactionNotificationSettings) -> Self {
        Self::ReactionNotificationSettings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNotification`] into [`Update`].
impl From<crate::types::UpdateNotification> for Update {
    fn from(val: crate::types::UpdateNotification) -> Self {
        Self::Notification(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNotificationGroup`] into [`Update`].
impl From<crate::types::UpdateNotificationGroup> for Update {
    fn from(val: crate::types::UpdateNotificationGroup) -> Self {
        Self::NotificationGroup(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateActiveNotifications`] into [`Update`].
impl From<crate::types::UpdateActiveNotifications> for Update {
    fn from(val: crate::types::UpdateActiveNotifications) -> Self {
        Self::ActiveNotifications(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateHavePendingNotifications`] into [`Update`].
impl From<crate::types::UpdateHavePendingNotifications> for Update {
    fn from(val: crate::types::UpdateHavePendingNotifications) -> Self {
        Self::HavePendingNotifications(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDeleteMessages`] into [`Update`].
impl From<crate::types::UpdateDeleteMessages> for Update {
    fn from(val: crate::types::UpdateDeleteMessages) -> Self {
        Self::DeleteMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatAction`] into [`Update`].
impl From<crate::types::UpdateChatAction> for Update {
    fn from(val: crate::types::UpdateChatAction) -> Self {
        Self::ChatAction(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdatePendingMessage`] into [`Update`].
impl From<crate::types::UpdatePendingMessage> for Update {
    fn from(val: crate::types::UpdatePendingMessage) -> Self {
        Self::PendingMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStopMessageDraft`] into [`Update`].
impl From<crate::types::UpdateStopMessageDraft> for Update {
    fn from(val: crate::types::UpdateStopMessageDraft) -> Self {
        Self::StopMessageDraft(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateCommunity`] into [`Update`].
impl From<crate::types::UpdateCommunity> for Update {
    fn from(val: crate::types::UpdateCommunity) -> Self {
        Self::Community(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUserStatus`] into [`Update`].
impl From<crate::types::UpdateUserStatus> for Update {
    fn from(val: crate::types::UpdateUserStatus) -> Self {
        Self::UserStatus(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUser`] into [`Update`].
impl From<crate::types::UpdateUser> for Update {
    fn from(val: crate::types::UpdateUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateBasicGroup`] into [`Update`].
impl From<crate::types::UpdateBasicGroup> for Update {
    fn from(val: crate::types::UpdateBasicGroup) -> Self {
        Self::BasicGroup(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSupergroup`] into [`Update`].
impl From<crate::types::UpdateSupergroup> for Update {
    fn from(val: crate::types::UpdateSupergroup) -> Self {
        Self::Supergroup(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSecretChat`] into [`Update`].
impl From<crate::types::UpdateSecretChat> for Update {
    fn from(val: crate::types::UpdateSecretChat) -> Self {
        Self::SecretChat(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUserFullInfo`] into [`Update`].
impl From<crate::types::UpdateUserFullInfo> for Update {
    fn from(val: crate::types::UpdateUserFullInfo) -> Self {
        Self::UserFullInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateBasicGroupFullInfo`] into [`Update`].
impl From<crate::types::UpdateBasicGroupFullInfo> for Update {
    fn from(val: crate::types::UpdateBasicGroupFullInfo) -> Self {
        Self::BasicGroupFullInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSupergroupFullInfo`] into [`Update`].
impl From<crate::types::UpdateSupergroupFullInfo> for Update {
    fn from(val: crate::types::UpdateSupergroupFullInfo) -> Self {
        Self::SupergroupFullInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateCommunityFullInfo`] into [`Update`].
impl From<crate::types::UpdateCommunityFullInfo> for Update {
    fn from(val: crate::types::UpdateCommunityFullInfo) -> Self {
        Self::CommunityFullInfo(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateServiceNotification`] into [`Update`].
impl From<crate::types::UpdateServiceNotification> for Update {
    fn from(val: crate::types::UpdateServiceNotification) -> Self {
        Self::ServiceNotification(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewOauthRequest`] into [`Update`].
impl From<crate::types::UpdateNewOauthRequest> for Update {
    fn from(val: crate::types::UpdateNewOauthRequest) -> Self {
        Self::NewOauthRequest(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFile`] into [`Update`].
impl From<crate::types::UpdateFile> for Update {
    fn from(val: crate::types::UpdateFile) -> Self {
        Self::File(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileGenerationStart`] into [`Update`].
impl From<crate::types::UpdateFileGenerationStart> for Update {
    fn from(val: crate::types::UpdateFileGenerationStart) -> Self {
        Self::FileGenerationStart(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileGenerationStop`] into [`Update`].
impl From<crate::types::UpdateFileGenerationStop> for Update {
    fn from(val: crate::types::UpdateFileGenerationStop) -> Self {
        Self::FileGenerationStop(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileDownloads`] into [`Update`].
impl From<crate::types::UpdateFileDownloads> for Update {
    fn from(val: crate::types::UpdateFileDownloads) -> Self {
        Self::FileDownloads(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileAddedToDownloads`] into [`Update`].
impl From<crate::types::UpdateFileAddedToDownloads> for Update {
    fn from(val: crate::types::UpdateFileAddedToDownloads) -> Self {
        Self::FileAddedToDownloads(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileDownload`] into [`Update`].
impl From<crate::types::UpdateFileDownload> for Update {
    fn from(val: crate::types::UpdateFileDownload) -> Self {
        Self::FileDownload(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFileRemovedFromDownloads`] into [`Update`].
impl From<crate::types::UpdateFileRemovedFromDownloads> for Update {
    fn from(val: crate::types::UpdateFileRemovedFromDownloads) -> Self {
        Self::FileRemovedFromDownloads(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateApplicationVerificationRequired`] into [`Update`].
impl From<crate::types::UpdateApplicationVerificationRequired> for Update {
    fn from(val: crate::types::UpdateApplicationVerificationRequired) -> Self {
        Self::ApplicationVerificationRequired(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateApplicationRecaptchaVerificationRequired`] into [`Update`].
impl From<crate::types::UpdateApplicationRecaptchaVerificationRequired> for Update {
    fn from(val: crate::types::UpdateApplicationRecaptchaVerificationRequired) -> Self {
        Self::ApplicationRecaptchaVerificationRequired(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateCall`] into [`Update`].
impl From<crate::types::UpdateCall> for Update {
    fn from(val: crate::types::UpdateCall) -> Self {
        Self::Call(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCall`] into [`Update`].
impl From<crate::types::UpdateGroupCall> for Update {
    fn from(val: crate::types::UpdateGroupCall) -> Self {
        Self::GroupCall(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallParticipant`] into [`Update`].
impl From<crate::types::UpdateGroupCallParticipant> for Update {
    fn from(val: crate::types::UpdateGroupCallParticipant) -> Self {
        Self::GroupCallParticipant(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallParticipants`] into [`Update`].
impl From<crate::types::UpdateGroupCallParticipants> for Update {
    fn from(val: crate::types::UpdateGroupCallParticipants) -> Self {
        Self::GroupCallParticipants(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallVerificationState`] into [`Update`].
impl From<crate::types::UpdateGroupCallVerificationState> for Update {
    fn from(val: crate::types::UpdateGroupCallVerificationState) -> Self {
        Self::GroupCallVerificationState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewGroupCallMessage`] into [`Update`].
impl From<crate::types::UpdateNewGroupCallMessage> for Update {
    fn from(val: crate::types::UpdateNewGroupCallMessage) -> Self {
        Self::NewGroupCallMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewGroupCallPaidReaction`] into [`Update`].
impl From<crate::types::UpdateNewGroupCallPaidReaction> for Update {
    fn from(val: crate::types::UpdateNewGroupCallPaidReaction) -> Self {
        Self::NewGroupCallPaidReaction(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallMessageSendFailed`] into [`Update`].
impl From<crate::types::UpdateGroupCallMessageSendFailed> for Update {
    fn from(val: crate::types::UpdateGroupCallMessageSendFailed) -> Self {
        Self::GroupCallMessageSendFailed(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallMessagesDeleted`] into [`Update`].
impl From<crate::types::UpdateGroupCallMessagesDeleted> for Update {
    fn from(val: crate::types::UpdateGroupCallMessagesDeleted) -> Self {
        Self::GroupCallMessagesDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateLiveStoryTopDonors`] into [`Update`].
impl From<crate::types::UpdateLiveStoryTopDonors> for Update {
    fn from(val: crate::types::UpdateLiveStoryTopDonors) -> Self {
        Self::LiveStoryTopDonors(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewCallSignalingData`] into [`Update`].
impl From<crate::types::UpdateNewCallSignalingData> for Update {
    fn from(val: crate::types::UpdateNewCallSignalingData) -> Self {
        Self::NewCallSignalingData(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGiftAuctionState`] into [`Update`].
impl From<crate::types::UpdateGiftAuctionState> for Update {
    fn from(val: crate::types::UpdateGiftAuctionState) -> Self {
        Self::GiftAuctionState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateActiveGiftAuctions`] into [`Update`].
impl From<crate::types::UpdateActiveGiftAuctions> for Update {
    fn from(val: crate::types::UpdateActiveGiftAuctions) -> Self {
        Self::ActiveGiftAuctions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUserPrivacySettingRules`] into [`Update`].
impl From<crate::types::UpdateUserPrivacySettingRules> for Update {
    fn from(val: crate::types::UpdateUserPrivacySettingRules) -> Self {
        Self::UserPrivacySettingRules(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUnreadMessageCount`] into [`Update`].
impl From<crate::types::UpdateUnreadMessageCount> for Update {
    fn from(val: crate::types::UpdateUnreadMessageCount) -> Self {
        Self::UnreadMessageCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUnreadChatCount`] into [`Update`].
impl From<crate::types::UpdateUnreadChatCount> for Update {
    fn from(val: crate::types::UpdateUnreadChatCount) -> Self {
        Self::UnreadChatCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatJoinResult`] into [`Update`].
impl From<crate::types::UpdateChatJoinResult> for Update {
    fn from(val: crate::types::UpdateChatJoinResult) -> Self {
        Self::ChatJoinResult(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStory`] into [`Update`].
impl From<crate::types::UpdateStory> for Update {
    fn from(val: crate::types::UpdateStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStoryDeleted`] into [`Update`].
impl From<crate::types::UpdateStoryDeleted> for Update {
    fn from(val: crate::types::UpdateStoryDeleted) -> Self {
        Self::StoryDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStoryPostSucceeded`] into [`Update`].
impl From<crate::types::UpdateStoryPostSucceeded> for Update {
    fn from(val: crate::types::UpdateStoryPostSucceeded) -> Self {
        Self::StoryPostSucceeded(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStoryPostFailed`] into [`Update`].
impl From<crate::types::UpdateStoryPostFailed> for Update {
    fn from(val: crate::types::UpdateStoryPostFailed) -> Self {
        Self::StoryPostFailed(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatActiveStories`] into [`Update`].
impl From<crate::types::UpdateChatActiveStories> for Update {
    fn from(val: crate::types::UpdateChatActiveStories) -> Self {
        Self::ChatActiveStories(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStoryListChatCount`] into [`Update`].
impl From<crate::types::UpdateStoryListChatCount> for Update {
    fn from(val: crate::types::UpdateStoryListChatCount) -> Self {
        Self::StoryListChatCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStoryStealthMode`] into [`Update`].
impl From<crate::types::UpdateStoryStealthMode> for Update {
    fn from(val: crate::types::UpdateStoryStealthMode) -> Self {
        Self::StoryStealthMode(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateTrustedMiniAppBots`] into [`Update`].
impl From<crate::types::UpdateTrustedMiniAppBots> for Update {
    fn from(val: crate::types::UpdateTrustedMiniAppBots) -> Self {
        Self::TrustedMiniAppBots(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateOption`] into [`Update`].
impl From<crate::types::UpdateOption> for Update {
    fn from(val: crate::types::UpdateOption) -> Self {
        Self::Option(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStickerSet`] into [`Update`].
impl From<crate::types::UpdateStickerSet> for Update {
    fn from(val: crate::types::UpdateStickerSet) -> Self {
        Self::StickerSet(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateInstalledStickerSets`] into [`Update`].
impl From<crate::types::UpdateInstalledStickerSets> for Update {
    fn from(val: crate::types::UpdateInstalledStickerSets) -> Self {
        Self::InstalledStickerSets(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateTrendingStickerSets`] into [`Update`].
impl From<crate::types::UpdateTrendingStickerSets> for Update {
    fn from(val: crate::types::UpdateTrendingStickerSets) -> Self {
        Self::TrendingStickerSets(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateRecentStickers`] into [`Update`].
impl From<crate::types::UpdateRecentStickers> for Update {
    fn from(val: crate::types::UpdateRecentStickers) -> Self {
        Self::RecentStickers(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFavoriteStickers`] into [`Update`].
impl From<crate::types::UpdateFavoriteStickers> for Update {
    fn from(val: crate::types::UpdateFavoriteStickers) -> Self {
        Self::FavoriteStickers(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSavedAnimations`] into [`Update`].
impl From<crate::types::UpdateSavedAnimations> for Update {
    fn from(val: crate::types::UpdateSavedAnimations) -> Self {
        Self::SavedAnimations(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSavedNotificationSounds`] into [`Update`].
impl From<crate::types::UpdateSavedNotificationSounds> for Update {
    fn from(val: crate::types::UpdateSavedNotificationSounds) -> Self {
        Self::SavedNotificationSounds(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDefaultBackground`] into [`Update`].
impl From<crate::types::UpdateDefaultBackground> for Update {
    fn from(val: crate::types::UpdateDefaultBackground) -> Self {
        Self::DefaultBackground(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateEmojiChatThemes`] into [`Update`].
impl From<crate::types::UpdateEmojiChatThemes> for Update {
    fn from(val: crate::types::UpdateEmojiChatThemes) -> Self {
        Self::EmojiChatThemes(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAccentColors`] into [`Update`].
impl From<crate::types::UpdateAccentColors> for Update {
    fn from(val: crate::types::UpdateAccentColors) -> Self {
        Self::AccentColors(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateProfileAccentColors`] into [`Update`].
impl From<crate::types::UpdateProfileAccentColors> for Update {
    fn from(val: crate::types::UpdateProfileAccentColors) -> Self {
        Self::ProfileAccentColors(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateWebBrowserSettings`] into [`Update`].
impl From<crate::types::UpdateWebBrowserSettings> for Update {
    fn from(val: crate::types::UpdateWebBrowserSettings) -> Self {
        Self::WebBrowserSettings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateLanguagePackStrings`] into [`Update`].
impl From<crate::types::UpdateLanguagePackStrings> for Update {
    fn from(val: crate::types::UpdateLanguagePackStrings) -> Self {
        Self::LanguagePackStrings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateConnectionState`] into [`Update`].
impl From<crate::types::UpdateConnectionState> for Update {
    fn from(val: crate::types::UpdateConnectionState) -> Self {
        Self::ConnectionState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateFreezeState`] into [`Update`].
impl From<crate::types::UpdateFreezeState> for Update {
    fn from(val: crate::types::UpdateFreezeState) -> Self {
        Self::FreezeState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAgeVerificationParameters`] into [`Update`].
impl From<crate::types::UpdateAgeVerificationParameters> for Update {
    fn from(val: crate::types::UpdateAgeVerificationParameters) -> Self {
        Self::AgeVerificationParameters(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateTermsOfService`] into [`Update`].
impl From<crate::types::UpdateTermsOfService> for Update {
    fn from(val: crate::types::UpdateTermsOfService) -> Self {
        Self::TermsOfService(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUnconfirmedSession`] into [`Update`].
impl From<crate::types::UpdateUnconfirmedSession> for Update {
    fn from(val: crate::types::UpdateUnconfirmedSession) -> Self {
        Self::UnconfirmedSession(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAttachmentMenuBots`] into [`Update`].
impl From<crate::types::UpdateAttachmentMenuBots> for Update {
    fn from(val: crate::types::UpdateAttachmentMenuBots) -> Self {
        Self::AttachmentMenuBots(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateWebAppMessageSent`] into [`Update`].
impl From<crate::types::UpdateWebAppMessageSent> for Update {
    fn from(val: crate::types::UpdateWebAppMessageSent) -> Self {
        Self::WebAppMessageSent(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateActiveEmojiReactions`] into [`Update`].
impl From<crate::types::UpdateActiveEmojiReactions> for Update {
    fn from(val: crate::types::UpdateActiveEmojiReactions) -> Self {
        Self::ActiveEmojiReactions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAvailableMessageEffects`] into [`Update`].
impl From<crate::types::UpdateAvailableMessageEffects> for Update {
    fn from(val: crate::types::UpdateAvailableMessageEffects) -> Self {
        Self::AvailableMessageEffects(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDefaultReactionType`] into [`Update`].
impl From<crate::types::UpdateDefaultReactionType> for Update {
    fn from(val: crate::types::UpdateDefaultReactionType) -> Self {
        Self::DefaultReactionType(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDefaultPaidReactionType`] into [`Update`].
impl From<crate::types::UpdateDefaultPaidReactionType> for Update {
    fn from(val: crate::types::UpdateDefaultPaidReactionType) -> Self {
        Self::DefaultPaidReactionType(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSavedMessagesTags`] into [`Update`].
impl From<crate::types::UpdateSavedMessagesTags> for Update {
    fn from(val: crate::types::UpdateSavedMessagesTags) -> Self {
        Self::SavedMessagesTags(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateActiveLiveLocationMessages`] into [`Update`].
impl From<crate::types::UpdateActiveLiveLocationMessages> for Update {
    fn from(val: crate::types::UpdateActiveLiveLocationMessages) -> Self {
        Self::ActiveLiveLocationMessages(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateOwnedStarCount`] into [`Update`].
impl From<crate::types::UpdateOwnedStarCount> for Update {
    fn from(val: crate::types::UpdateOwnedStarCount) -> Self {
        Self::OwnedStarCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateOwnedGramCount`] into [`Update`].
impl From<crate::types::UpdateOwnedGramCount> for Update {
    fn from(val: crate::types::UpdateOwnedGramCount) -> Self {
        Self::OwnedGramCount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatRevenueAmount`] into [`Update`].
impl From<crate::types::UpdateChatRevenueAmount> for Update {
    fn from(val: crate::types::UpdateChatRevenueAmount) -> Self {
        Self::ChatRevenueAmount(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStarRevenueStatus`] into [`Update`].
impl From<crate::types::UpdateStarRevenueStatus> for Update {
    fn from(val: crate::types::UpdateStarRevenueStatus) -> Self {
        Self::StarRevenueStatus(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGramRevenueStatus`] into [`Update`].
impl From<crate::types::UpdateGramRevenueStatus> for Update {
    fn from(val: crate::types::UpdateGramRevenueStatus) -> Self {
        Self::GramRevenueStatus(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSpeechRecognitionTrial`] into [`Update`].
impl From<crate::types::UpdateSpeechRecognitionTrial> for Update {
    fn from(val: crate::types::UpdateSpeechRecognitionTrial) -> Self {
        Self::SpeechRecognitionTrial(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateGroupCallMessageLevels`] into [`Update`].
impl From<crate::types::UpdateGroupCallMessageLevels> for Update {
    fn from(val: crate::types::UpdateGroupCallMessageLevels) -> Self {
        Self::GroupCallMessageLevels(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateDiceEmojis`] into [`Update`].
impl From<crate::types::UpdateDiceEmojis> for Update {
    fn from(val: crate::types::UpdateDiceEmojis) -> Self {
        Self::DiceEmojis(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateStakeDiceState`] into [`Update`].
impl From<crate::types::UpdateStakeDiceState> for Update {
    fn from(val: crate::types::UpdateStakeDiceState) -> Self {
        Self::StakeDiceState(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAnimatedEmojiMessageClicked`] into [`Update`].
impl From<crate::types::UpdateAnimatedEmojiMessageClicked> for Update {
    fn from(val: crate::types::UpdateAnimatedEmojiMessageClicked) -> Self {
        Self::AnimatedEmojiMessageClicked(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAnimationSearchParameters`] into [`Update`].
impl From<crate::types::UpdateAnimationSearchParameters> for Update {
    fn from(val: crate::types::UpdateAnimationSearchParameters) -> Self {
        Self::AnimationSearchParameters(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateTextCompositionStyles`] into [`Update`].
impl From<crate::types::UpdateTextCompositionStyles> for Update {
    fn from(val: crate::types::UpdateTextCompositionStyles) -> Self {
        Self::TextCompositionStyles(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSuggestedActions`] into [`Update`].
impl From<crate::types::UpdateSuggestedActions> for Update {
    fn from(val: crate::types::UpdateSuggestedActions) -> Self {
        Self::SuggestedActions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateSpeedLimitNotification`] into [`Update`].
impl From<crate::types::UpdateSpeedLimitNotification> for Update {
    fn from(val: crate::types::UpdateSpeedLimitNotification) -> Self {
        Self::SpeedLimitNotification(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateContactCloseBirthdays`] into [`Update`].
impl From<crate::types::UpdateContactCloseBirthdays> for Update {
    fn from(val: crate::types::UpdateContactCloseBirthdays) -> Self {
        Self::ContactCloseBirthdays(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateAutosaveSettings`] into [`Update`].
impl From<crate::types::UpdateAutosaveSettings> for Update {
    fn from(val: crate::types::UpdateAutosaveSettings) -> Self {
        Self::AutosaveSettings(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateBusinessConnection`] into [`Update`].
impl From<crate::types::UpdateBusinessConnection> for Update {
    fn from(val: crate::types::UpdateBusinessConnection) -> Self {
        Self::BusinessConnection(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewBusinessMessage`] into [`Update`].
impl From<crate::types::UpdateNewBusinessMessage> for Update {
    fn from(val: crate::types::UpdateNewBusinessMessage) -> Self {
        Self::NewBusinessMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateBusinessMessageEdited`] into [`Update`].
impl From<crate::types::UpdateBusinessMessageEdited> for Update {
    fn from(val: crate::types::UpdateBusinessMessageEdited) -> Self {
        Self::BusinessMessageEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateBusinessMessagesDeleted`] into [`Update`].
impl From<crate::types::UpdateBusinessMessagesDeleted> for Update {
    fn from(val: crate::types::UpdateBusinessMessagesDeleted) -> Self {
        Self::BusinessMessagesDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewInlineQuery`] into [`Update`].
impl From<crate::types::UpdateNewInlineQuery> for Update {
    fn from(val: crate::types::UpdateNewInlineQuery) -> Self {
        Self::NewInlineQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewChosenInlineResult`] into [`Update`].
impl From<crate::types::UpdateNewChosenInlineResult> for Update {
    fn from(val: crate::types::UpdateNewChosenInlineResult) -> Self {
        Self::NewChosenInlineResult(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewGuestQuery`] into [`Update`].
impl From<crate::types::UpdateNewGuestQuery> for Update {
    fn from(val: crate::types::UpdateNewGuestQuery) -> Self {
        Self::NewGuestQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewCallbackQuery`] into [`Update`].
impl From<crate::types::UpdateNewCallbackQuery> for Update {
    fn from(val: crate::types::UpdateNewCallbackQuery) -> Self {
        Self::NewCallbackQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewInlineCallbackQuery`] into [`Update`].
impl From<crate::types::UpdateNewInlineCallbackQuery> for Update {
    fn from(val: crate::types::UpdateNewInlineCallbackQuery) -> Self {
        Self::NewInlineCallbackQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewBusinessCallbackQuery`] into [`Update`].
impl From<crate::types::UpdateNewBusinessCallbackQuery> for Update {
    fn from(val: crate::types::UpdateNewBusinessCallbackQuery) -> Self {
        Self::NewBusinessCallbackQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewShippingQuery`] into [`Update`].
impl From<crate::types::UpdateNewShippingQuery> for Update {
    fn from(val: crate::types::UpdateNewShippingQuery) -> Self {
        Self::NewShippingQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewPreCheckoutQuery`] into [`Update`].
impl From<crate::types::UpdateNewPreCheckoutQuery> for Update {
    fn from(val: crate::types::UpdateNewPreCheckoutQuery) -> Self {
        Self::NewPreCheckoutQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewCustomEvent`] into [`Update`].
impl From<crate::types::UpdateNewCustomEvent> for Update {
    fn from(val: crate::types::UpdateNewCustomEvent) -> Self {
        Self::NewCustomEvent(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewCustomQuery`] into [`Update`].
impl From<crate::types::UpdateNewCustomQuery> for Update {
    fn from(val: crate::types::UpdateNewCustomQuery) -> Self {
        Self::NewCustomQuery(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateUserSubscription`] into [`Update`].
impl From<crate::types::UpdateUserSubscription> for Update {
    fn from(val: crate::types::UpdateUserSubscription) -> Self {
        Self::UserSubscription(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdatePoll`] into [`Update`].
impl From<crate::types::UpdatePoll> for Update {
    fn from(val: crate::types::UpdatePoll) -> Self {
        Self::Poll(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdatePollAnswer`] into [`Update`].
impl From<crate::types::UpdatePollAnswer> for Update {
    fn from(val: crate::types::UpdatePollAnswer) -> Self {
        Self::PollAnswer(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateManagedBot`] into [`Update`].
impl From<crate::types::UpdateManagedBot> for Update {
    fn from(val: crate::types::UpdateManagedBot) -> Self {
        Self::ManagedBot(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatMember`] into [`Update`].
impl From<crate::types::UpdateChatMember> for Update {
    fn from(val: crate::types::UpdateChatMember) -> Self {
        Self::ChatMember(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateNewChatJoinRequest`] into [`Update`].
impl From<crate::types::UpdateNewChatJoinRequest> for Update {
    fn from(val: crate::types::UpdateNewChatJoinRequest) -> Self {
        Self::NewChatJoinRequest(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateChatBoost`] into [`Update`].
impl From<crate::types::UpdateChatBoost> for Update {
    fn from(val: crate::types::UpdateChatBoost) -> Self {
        Self::ChatBoost(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageReaction`] into [`Update`].
impl From<crate::types::UpdateMessageReaction> for Update {
    fn from(val: crate::types::UpdateMessageReaction) -> Self {
        Self::MessageReaction(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdateMessageReactions`] into [`Update`].
impl From<crate::types::UpdateMessageReactions> for Update {
    fn from(val: crate::types::UpdateMessageReactions) -> Self {
        Self::MessageReactions(Box::new(val))
    }
}

/// Converts a [`crate::types::UpdatePaidMediaPurchased`] into [`Update`].
impl From<crate::types::UpdatePaidMediaPurchased> for Update {
    fn from(val: crate::types::UpdatePaidMediaPurchased) -> Self {
        Self::PaidMediaPurchased(Box::new(val))
    }
}

/// TDLib `Updates` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Updates {
    /// Contains a list of updates
    #[serde(rename(serialize = "updates", deserialize = "updates"))]
    Updates(Box<crate::types::Updates>),
}

impl Updates {
    /// Convenience constructor to create a [`Updates::Updates`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn updates(val: crate::types::Updates) -> Self {
        Self::Updates(Box::new(val))
    }

}

/// Converts a [`crate::types::Updates`] into [`Updates`].
impl From<crate::types::Updates> for Updates {
    fn from(val: crate::types::Updates) -> Self {
        Self::Updates(Box::new(val))
    }
}

/// Describes a stream to which TDLib internal log is written
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogStream {
    /// The log is written to stderr or an OS specific log
    #[serde(rename(serialize = "logStreamDefault", deserialize = "logStreamDefault"))]
    Default,
    /// The log is written to a file
    #[serde(rename(serialize = "logStreamFile", deserialize = "logStreamFile"))]
    File(Box<crate::types::LogStreamFile>),
    /// The log is written nowhere
    #[serde(rename(serialize = "logStreamEmpty", deserialize = "logStreamEmpty"))]
    Empty,
}

impl LogStream {
    /// Convenience constructor to create a [`LogStream::File`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file(val: crate::types::LogStreamFile) -> Self {
        Self::File(Box::new(val))
    }

}

/// Converts a [`crate::types::LogStreamFile`] into [`LogStream`].
impl From<crate::types::LogStreamFile> for LogStream {
    fn from(val: crate::types::LogStreamFile) -> Self {
        Self::File(Box::new(val))
    }
}

/// TDLib `LogVerbosityLevel` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogVerbosityLevel {
    /// Contains a TDLib internal log verbosity level
    #[serde(rename(serialize = "logVerbosityLevel", deserialize = "logVerbosityLevel"))]
    LogVerbosityLevel(Box<crate::types::LogVerbosityLevel>),
}

impl LogVerbosityLevel {
    /// Convenience constructor to create a [`LogVerbosityLevel::LogVerbosityLevel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn log_verbosity_level(val: crate::types::LogVerbosityLevel) -> Self {
        Self::LogVerbosityLevel(Box::new(val))
    }

}

/// Converts a [`crate::types::LogVerbosityLevel`] into [`LogVerbosityLevel`].
impl From<crate::types::LogVerbosityLevel> for LogVerbosityLevel {
    fn from(val: crate::types::LogVerbosityLevel) -> Self {
        Self::LogVerbosityLevel(Box::new(val))
    }
}

/// TDLib `LogTags` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogTags {
    /// Contains a list of available TDLib internal log tags
    #[serde(rename(serialize = "logTags", deserialize = "logTags"))]
    LogTags(Box<crate::types::LogTags>),
}

impl LogTags {
    /// Convenience constructor to create a [`LogTags::LogTags`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn log_tags(val: crate::types::LogTags) -> Self {
        Self::LogTags(Box::new(val))
    }

}

/// Converts a [`crate::types::LogTags`] into [`LogTags`].
impl From<crate::types::LogTags> for LogTags {
    fn from(val: crate::types::LogTags) -> Self {
        Self::LogTags(Box::new(val))
    }
}

/// TDLib `TestInt` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestInt {
    /// A simple object containing a number; for testing only
    #[serde(rename(serialize = "testInt", deserialize = "testInt"))]
    TestInt(Box<crate::types::TestInt>),
}

impl TestInt {
    /// Convenience constructor to create a [`TestInt::TestInt`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_int(val: crate::types::TestInt) -> Self {
        Self::TestInt(Box::new(val))
    }

}

/// Converts a [`crate::types::TestInt`] into [`TestInt`].
impl From<crate::types::TestInt> for TestInt {
    fn from(val: crate::types::TestInt) -> Self {
        Self::TestInt(Box::new(val))
    }
}

/// TDLib `TestString` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestString {
    /// A simple object containing a string; for testing only
    #[serde(rename(serialize = "testString", deserialize = "testString"))]
    TestString(Box<crate::types::TestString>),
}

impl TestString {
    /// Convenience constructor to create a [`TestString::TestString`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_string(val: crate::types::TestString) -> Self {
        Self::TestString(Box::new(val))
    }

}

/// Converts a [`crate::types::TestString`] into [`TestString`].
impl From<crate::types::TestString> for TestString {
    fn from(val: crate::types::TestString) -> Self {
        Self::TestString(Box::new(val))
    }
}

/// TDLib `TestBytes` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestBytes {
    /// A simple object containing a sequence of bytes; for testing only
    #[serde(rename(serialize = "testBytes", deserialize = "testBytes"))]
    TestBytes(Box<crate::types::TestBytes>),
}

impl TestBytes {
    /// Convenience constructor to create a [`TestBytes::TestBytes`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_bytes(val: crate::types::TestBytes) -> Self {
        Self::TestBytes(Box::new(val))
    }

}

/// Converts a [`crate::types::TestBytes`] into [`TestBytes`].
impl From<crate::types::TestBytes> for TestBytes {
    fn from(val: crate::types::TestBytes) -> Self {
        Self::TestBytes(Box::new(val))
    }
}

/// TDLib `TestVectorInt` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorInt {
    /// A simple object containing a vector of numbers; for testing only
    #[serde(rename(serialize = "testVectorInt", deserialize = "testVectorInt"))]
    TestVectorInt(Box<crate::types::TestVectorInt>),
}

impl TestVectorInt {
    /// Convenience constructor to create a [`TestVectorInt::TestVectorInt`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_vector_int(val: crate::types::TestVectorInt) -> Self {
        Self::TestVectorInt(Box::new(val))
    }

}

/// Converts a [`crate::types::TestVectorInt`] into [`TestVectorInt`].
impl From<crate::types::TestVectorInt> for TestVectorInt {
    fn from(val: crate::types::TestVectorInt) -> Self {
        Self::TestVectorInt(Box::new(val))
    }
}

/// TDLib `TestVectorIntObject` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorIntObject {
    /// A simple object containing a vector of objects that hold a number; for testing only
    #[serde(rename(serialize = "testVectorIntObject", deserialize = "testVectorIntObject"))]
    TestVectorIntObject(Box<crate::types::TestVectorIntObject>),
}

impl TestVectorIntObject {
    /// Convenience constructor to create a [`TestVectorIntObject::TestVectorIntObject`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_vector_int_object(val: crate::types::TestVectorIntObject) -> Self {
        Self::TestVectorIntObject(Box::new(val))
    }

}

/// Converts a [`crate::types::TestVectorIntObject`] into [`TestVectorIntObject`].
impl From<crate::types::TestVectorIntObject> for TestVectorIntObject {
    fn from(val: crate::types::TestVectorIntObject) -> Self {
        Self::TestVectorIntObject(Box::new(val))
    }
}

/// TDLib `TestVectorString` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorString {
    /// A simple object containing a vector of strings; for testing only
    #[serde(rename(serialize = "testVectorString", deserialize = "testVectorString"))]
    TestVectorString(Box<crate::types::TestVectorString>),
}

impl TestVectorString {
    /// Convenience constructor to create a [`TestVectorString::TestVectorString`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_vector_string(val: crate::types::TestVectorString) -> Self {
        Self::TestVectorString(Box::new(val))
    }

}

/// Converts a [`crate::types::TestVectorString`] into [`TestVectorString`].
impl From<crate::types::TestVectorString> for TestVectorString {
    fn from(val: crate::types::TestVectorString) -> Self {
        Self::TestVectorString(Box::new(val))
    }
}

/// TDLib `TestVectorStringObject` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorStringObject {
    /// A simple object containing a vector of objects that hold a string; for testing only
    #[serde(rename(serialize = "testVectorStringObject", deserialize = "testVectorStringObject"))]
    TestVectorStringObject(Box<crate::types::TestVectorStringObject>),
}

impl TestVectorStringObject {
    /// Convenience constructor to create a [`TestVectorStringObject::TestVectorStringObject`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn test_vector_string_object(val: crate::types::TestVectorStringObject) -> Self {
        Self::TestVectorStringObject(Box::new(val))
    }

}

/// Converts a [`crate::types::TestVectorStringObject`] into [`TestVectorStringObject`].
impl From<crate::types::TestVectorStringObject> for TestVectorStringObject {
    fn from(val: crate::types::TestVectorStringObject) -> Self {
        Self::TestVectorStringObject(Box::new(val))
    }
}

