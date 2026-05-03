#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FirebaseAuthenticationSettings {
    /// Settings for Firebase Authentication in the official Android application
    #[serde(rename(
        serialize = "firebaseAuthenticationSettingsAndroid",
        deserialize = "firebaseAuthenticationSettingsAndroid"
    ))]
    Android,
    /// Settings for Firebase Authentication in the official iOS application
    #[serde(rename(
        serialize = "firebaseAuthenticationSettingsIos",
        deserialize = "firebaseAuthenticationSettingsIos"
    ))]
    Ios(crate::types::FirebaseAuthenticationSettingsIos),
}
