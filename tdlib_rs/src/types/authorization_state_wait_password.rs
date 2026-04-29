#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user has been authorized, but needs to enter a 2-step verification password to start using the application.
/// Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AuthorizationStateWaitPassword {
    /// Hint for the password; may be empty
    pub password_hint: String,
    /// True, if a recovery email address has been set up
    pub has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved
    pub has_passport_data: bool,
    /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
    pub recovery_email_address_pattern: String,
}
