#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A token for Ubuntu Push Client service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenUbuntuPush {
    /// Token; may be empty to deregister a device
    pub token: String,
}
