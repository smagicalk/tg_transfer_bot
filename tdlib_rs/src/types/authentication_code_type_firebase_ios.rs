#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFirebaseIos {
    /// Receipt of successful application token validation to compare with receipt from push notification
    pub receipt: String,
    /// Time after the next authentication method is expected to be used if verification push notification isn't received, in seconds
    pub push_timeout: i32,
    /// Length of the code
    pub length: i32,
}
