#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A digit-only authentication code is delivered via a phone call to the specified phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeCall {
    /// Length of the code
    pub length: i32,
}
