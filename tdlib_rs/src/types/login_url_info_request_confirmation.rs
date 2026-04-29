#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An authorization confirmation dialog needs to be shown to the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LoginUrlInfoRequestConfirmation {
    /// An HTTP URL to be opened
    pub url: String,
    /// A domain of the URL
    pub domain: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
}
