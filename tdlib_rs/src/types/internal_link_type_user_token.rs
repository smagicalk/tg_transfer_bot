#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link.
/// If the user is found, then call createPrivateChat and open the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUserToken {
    /// The token
    pub token: String,
}
