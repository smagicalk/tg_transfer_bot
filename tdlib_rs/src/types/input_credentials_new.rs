#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Applies if a user enters new credentials on a payment provider website
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsNew {
    /// JSON-encoded data with the credential identifier from the payment provider
    pub data: String,
    /// True, if the credential identifier can be saved on the server side
    pub allow_save: bool,
}
