#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeMissedCall {
    /// Prefix of the phone number from which the call will be made
    pub phone_number_prefix: String,
    /// Number of digits in the code, excluding the prefix
    pub length: i32,
}
