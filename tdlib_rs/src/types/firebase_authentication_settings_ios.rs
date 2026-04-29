#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Settings for Firebase Authentication in the official iOS application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FirebaseAuthenticationSettingsIos {
    /// Device token from Apple Push Notification service
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
}
