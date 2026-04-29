#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputCredentials {
    /// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
    #[serde(rename(serialize = "inputCredentialsSaved", deserialize = "inputCredentialsSaved"))]
    Saved(crate::types::InputCredentialsSaved),
    /// Applies if a user enters new credentials on a payment provider website
    #[serde(rename(serialize = "inputCredentialsNew", deserialize = "inputCredentialsNew"))]
    New(crate::types::InputCredentialsNew),
    /// Applies if a user enters new credentials using Apple Pay
    #[serde(rename(serialize = "inputCredentialsApplePay", deserialize = "inputCredentialsApplePay"))]
    ApplePay(crate::types::InputCredentialsApplePay),
    /// Applies if a user enters new credentials using Google Pay
    #[serde(rename(serialize = "inputCredentialsGooglePay", deserialize = "inputCredentialsGooglePay"))]
    GooglePay(crate::types::InputCredentialsGooglePay),
}
