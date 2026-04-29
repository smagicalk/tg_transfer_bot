#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthenticationCodeTypeFirebaseAndroid {
    /// Parameters to be used for device verification
    pub device_verification_parameters: crate::enums::FirebaseDeviceVerificationParameters,
    /// Length of the code
    pub length: i32,
}
