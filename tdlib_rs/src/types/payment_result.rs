#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains the result of a payment request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentResult {
    /// True, if the payment request was successful; otherwise, the verification_url will be non-empty
    pub success: bool,
    /// URL for additional payment credentials verification
    pub verification_url: String,
}
