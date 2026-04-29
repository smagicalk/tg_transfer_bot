#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A button that opens a specified URL and automatically authorize the current user by calling getLoginUrlInfo
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
    /// An HTTP URL to pass to getLoginUrlInfo
    pub url: String,
    /// Unique button identifier
    pub id: i64,
    /// If non-empty, new text of the button in forwarded messages
    pub forward_text: String,
}
