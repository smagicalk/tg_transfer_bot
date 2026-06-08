#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A token for Firebase Cloud Messaging
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
    /// Device registration token; may be empty to deregister a device
    pub token: String,
    /// True, if push notifications must be additionally encrypted
    pub encrypt: bool,
}
