#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes an additional payment option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentOption {
    /// Title for the payment option
    pub title: String,
    /// Payment form URL to be opened in a web view
    pub url: String,
}
