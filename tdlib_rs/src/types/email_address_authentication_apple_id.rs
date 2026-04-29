#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authentication token received through Apple ID
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationAppleId {
    /// The token
    pub token: String,
}
