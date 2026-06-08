#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A token for Apple Push Notification service VoIP notifications
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenApplePushVoIp {
    /// Device token; may be empty to deregister a device
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
    /// True, if push notifications must be additionally encrypted
    pub encrypt: bool,
}
