#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an action associated with a bank card number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BankCardActionOpenUrl {
    /// Action text
    pub text: String,
    /// The URL to be opened
    pub url: String,
}
