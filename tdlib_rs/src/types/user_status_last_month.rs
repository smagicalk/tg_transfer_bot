#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user is offline, but was online last month
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusLastMonth {
    /// Exact user's status is hidden because the current user enabled userPrivacySettingShowStatus privacy setting for the user and has no Telegram Premium
    pub by_my_privacy_settings: bool,
}
