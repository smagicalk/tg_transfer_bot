#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A token for Tizen Push Service
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenTizenPush {
    /// Push service registration identifier; may be empty to deregister a device
    pub reg_id: String,
}
