#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Smart Glocal payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderSmartGlocal {
    /// Public payment token
    pub public_token: String,
    /// URL for sending card tokenization requests
    pub tokenize_url: String,
}
