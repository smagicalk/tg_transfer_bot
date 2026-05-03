#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
    /// A tg: URL for the QR code. The link will be updated frequently
    pub link: String,
}
