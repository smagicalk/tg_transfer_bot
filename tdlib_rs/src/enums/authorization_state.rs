#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AuthorizationState {
    /// Initialization parameters are needed. Call setTdlibParameters to provide them
    #[serde(rename(serialize = "authorizationStateWaitTdlibParameters", deserialize = "authorizationStateWaitTdlibParameters"))]
    WaitTdlibParameters,
    /// TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number,
    /// or use requestQrCodeAuthentication, getAuthenticationPasskeyParameters, or checkAuthenticationBotToken for other authentication options
    #[serde(rename(serialize = "authorizationStateWaitPhoneNumber", deserialize = "authorizationStateWaitPhoneNumber"))]
    WaitPhoneNumber,
    /// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
    #[serde(rename(serialize = "authorizationStateWaitPremiumPurchase", deserialize = "authorizationStateWaitPremiumPurchase"))]
    WaitPremiumPurchase(crate::types::AuthorizationStateWaitPremiumPurchase),
    /// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
    #[serde(rename(serialize = "authorizationStateWaitEmailAddress", deserialize = "authorizationStateWaitEmailAddress"))]
    WaitEmailAddress(crate::types::AuthorizationStateWaitEmailAddress),
    /// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
    #[serde(rename(serialize = "authorizationStateWaitEmailCode", deserialize = "authorizationStateWaitEmailCode"))]
    WaitEmailCode(crate::types::AuthorizationStateWaitEmailCode),
    /// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
    #[serde(rename(serialize = "authorizationStateWaitCode", deserialize = "authorizationStateWaitCode"))]
    WaitCode(crate::types::AuthorizationStateWaitCode),
    /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
    #[serde(rename(serialize = "authorizationStateWaitOtherDeviceConfirmation", deserialize = "authorizationStateWaitOtherDeviceConfirmation"))]
    WaitOtherDeviceConfirmation(crate::types::AuthorizationStateWaitOtherDeviceConfirmation),
    /// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
    #[serde(rename(serialize = "authorizationStateWaitRegistration", deserialize = "authorizationStateWaitRegistration"))]
    WaitRegistration(crate::types::AuthorizationStateWaitRegistration),
    /// The user has been authorized, but needs to enter a 2-step verification password to start using the application.
    /// Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
    #[serde(rename(serialize = "authorizationStateWaitPassword", deserialize = "authorizationStateWaitPassword"))]
    WaitPassword(crate::types::AuthorizationStateWaitPassword),
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
