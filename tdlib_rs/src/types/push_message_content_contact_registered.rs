#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A contact has registered with Telegram
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentContactRegistered {
    /// True, if the user joined Telegram as a Telegram Premium account
    pub as_premium_account: bool,
}
