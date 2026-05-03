#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user was online recently
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UserStatusRecently {
    /// Exact user's status is hidden because the current user enabled userPrivacySettingShowStatus privacy setting for the user and has no Telegram Premium
    pub by_my_privacy_settings: bool,
}
