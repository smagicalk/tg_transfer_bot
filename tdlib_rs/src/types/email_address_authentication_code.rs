#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authentication code delivered to a user's email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationCode {
    /// The code
    pub code: String,
}
