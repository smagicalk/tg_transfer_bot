#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link.
/// If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePhoneNumberConfirmation {
    /// Hash value from the link
    pub hash: String,
    /// Phone number value from the link
    pub phone_number: String,
}
