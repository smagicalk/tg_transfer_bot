#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents the current state of 2-step verification
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PasswordState {
    /// True, if a 2-step verification password is set
    pub has_password: bool,
    /// Hint for the password; may be empty
    pub password_hint: String,
    /// True, if a recovery email is set
    pub has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved
    pub has_passport_data: bool,
    /// Information about the recovery email address to which the confirmation email was sent; may be null
    pub recovery_email_address_code_info: Option<crate::types::EmailAddressAuthenticationCodeInfo>,
    /// Pattern of the email address set up for logging in
    pub login_email_address_pattern: String,
    /// If not 0, point in time (Unix timestamp) after which the 2-step verification password can be reset immediately using resetPassword
    pub pending_reset_date: i32,
}
