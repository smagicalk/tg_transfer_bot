#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A token for web Push API
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DeviceTokenWebPush {
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
    pub endpoint: String,
    /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key
    pub p256dh_base64url: String,
    /// Base64url-encoded authentication secret
    pub auth_base64url: String,
}
