#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that requests users to be shared by the current user; available only in private chats. Use the method shareUsersWithBot to complete the request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyboardButtonTypeRequestUsers {
    /// Unique button identifier
    pub id: i32,
    /// True, if the shared users must or must not be bots
    pub restrict_user_is_bot: bool,
    /// True, if the shared users must be bots; otherwise, the shared users must not be bots. Ignored if restrict_user_is_bot is false
    pub user_is_bot: bool,
    /// True, if the shared users must or must not be Telegram Premium users
    pub restrict_user_is_premium: bool,
    /// True, if the shared users must be Telegram Premium users; otherwise, the shared users must not be Telegram Premium users. Ignored if restrict_user_is_premium is false
    pub user_is_premium: bool,
    /// The maximum number of users to share
    pub max_quantity: i32,
    /// Pass true to request name of the users; bots only
    pub request_name: bool,
    /// Pass true to request username of the users; bots only
    pub request_username: bool,
    /// Pass true to request photo of the users; bots only
    pub request_photo: bool,
}
