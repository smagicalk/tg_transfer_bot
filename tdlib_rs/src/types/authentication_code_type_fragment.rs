#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A digit-only authentication code is delivered to https:fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFragment {
    /// URL to open to receive the code
    pub url: String,
    /// Length of the code
    pub length: i32,
}
