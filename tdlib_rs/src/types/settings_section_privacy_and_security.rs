#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The privacy and security section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionPrivacyAndSecurity {
    /// Subsection of the section; may be one of
    /// "", "blocked", "blocked/edit", "blocked/block-user", "blocked/block-user/chats", "blocked/block-user/contacts",
    /// "active-websites", "active-websites/edit", "active-websites/disconnect-all", "passcode", "passcode/disable",
    /// "passcode/change", "passcode/auto-lock", "passcode/face-id", "passcode/fingerprint", "2sv", "2sv/change",
    /// "2sv/disable", "2sv/change-email", "passkey", "passkey/create", "auto-delete", "auto-delete/set-custom",
    /// "login-email", "phone-number", "phone-number/never", "phone-number/always", "last-seen", "last-seen/never",
    /// "last-seen/always", "last-seen/hide-read-time", "profile-photos", "profile-photos/never", "profile-photos/always",
    /// "profile-photos/set-public", "profile-photos/update-public", "profile-photos/remove-public", "bio", "bio/never",
    /// "bio/always", "gifts", "gifts/show-icon", "gifts/never", "gifts/always", "gifts/accepted-types", "birthday",
    /// "birthday/add", "birthday/never", "birthday/always", "saved-music", "saved-music/never", "saved-music/always",
    /// "forwards", "forwards/never", "forwards/always", "calls", "calls/never", "calls/always", "calls/p2p",
    /// "calls/p2p/never", "calls/p2p/always", "calls/ios-integration", "voice", "voice/never", "voice/always",
    /// "messages", "messages/set-price", "messages/exceptions", "invites", "invites/never", "invites/always",
    /// "self-destruct", "data-settings", "data-settings/sync-contacts", "data-settings/delete-synced",
    /// "data-settings/suggest-contacts", "data-settings/delete-cloud-drafts", "data-settings/clear-payment-info",
    /// "data-settings/link-previews", "data-settings/bot-settings", "data-settings/map-provider", "archive-and-mute"
    pub subsection: String,
}
