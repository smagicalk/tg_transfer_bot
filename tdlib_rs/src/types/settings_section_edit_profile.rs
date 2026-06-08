#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The profile edit section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionEditProfile {
    /// Subsection of the section; may be one of
    /// "", "set-photo", "first-name", "last-name", "emoji-status", "bio", "birthday", "change-number", "username",
    /// "your-color", "channel", "add-account", "log-out", "profile-color/profile", "profile-color/profile/add-icons",
    /// "profile-color/profile/use-gift", "profile-color/name", "profile-color/name/add-icons",
    /// "profile-color/name/use-gift", "profile-photo/use-emoji"
    pub subsection: String,
}
