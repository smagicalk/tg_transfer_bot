#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains settings for the authentication of the user's phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberAuthenticationSettings {
    /// Pass true if the authentication code may be sent via a flash call to the specified phone number
    pub allow_flash_call: bool,
    /// Pass true if the authentication code may be sent via a missed call to the specified phone number
    pub allow_missed_call: bool,
    /// Pass true if the authenticated phone number is used on the current device
    pub is_current_phone_number: bool,
    /// Pass true if there is a SIM card in the current device, but it is not possible to check whether phone number matches
    pub has_unknown_phone_number: bool,
    /// For official applications only. True, if the application can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https:developers.google.com/identity/sms-retriever/ for more details
    pub allow_sms_retriever_api: bool,
    /// For official Android and iOS applications only; pass null otherwise. Settings for Firebase Authentication
    pub firebase_authentication_settings: Option<crate::enums::FirebaseAuthenticationSettings>,
    /// List of up to 20 authentication tokens, recently received in updateOption("authentication_token") in previously logged out sessions; for setAuthenticationPhoneNumber only
    pub authentication_tokens: Vec<String>,
}
