#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Some other payment provider, for which a web payment form must be shown
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderOther {
    /// Payment form URL
    pub url: String,
}
