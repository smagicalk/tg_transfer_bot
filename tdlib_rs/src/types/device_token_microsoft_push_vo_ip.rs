#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenMicrosoftPushVoIp {
    /// Push notification channel URI; may be empty to deregister a device
    pub channel_uri: String,
}
