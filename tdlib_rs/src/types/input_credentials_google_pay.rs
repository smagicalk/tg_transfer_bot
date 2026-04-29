#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Applies if a user enters new credentials using Google Pay
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsGooglePay {
    /// JSON-encoded data with the credential identifier
    pub data: String,
}
