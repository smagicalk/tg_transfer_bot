#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A token for Simple Push API for Firefox OS
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenSimplePush {
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
    pub endpoint: String,
}
