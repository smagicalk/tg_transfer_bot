#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
    /// Length of the code
    pub length: i32,
}
