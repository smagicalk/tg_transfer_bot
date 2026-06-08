#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A token for Apple Push Notification service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenApplePush {
    /// Device token; may be empty to deregister a device
    pub device_token: String,
    /// True, if App Sandbox is enabled
    pub is_app_sandbox: bool,
}
