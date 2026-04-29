#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link.
/// If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeUserPhoneNumber {
    /// Phone number of the user
    pub phone_number: String,
    /// Draft text for message to send in the chat
    pub draft_text: String,
    /// True, if user's profile information screen must be opened; otherwise, the chat itself must be opened
    pub open_profile: bool,
}
