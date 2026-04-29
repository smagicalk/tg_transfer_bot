#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authentication token received through Google ID
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EmailAddressAuthenticationGoogleId {
    /// The token
    pub token: String,
}
