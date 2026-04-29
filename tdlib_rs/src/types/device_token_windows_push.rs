#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A token for Windows Push Notification Services
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenWindowsPush {
    /// The access token that will be used to send notifications; may be empty to deregister a device
    pub access_token: String,
}
