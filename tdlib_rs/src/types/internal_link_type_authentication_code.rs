#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeAuthenticationCode {
    /// The authentication code
    pub code: String,
}
