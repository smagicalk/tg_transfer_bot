#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeSms {
    /// Length of the code
    pub length: i32,
}
