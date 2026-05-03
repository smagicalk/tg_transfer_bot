#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The password reset request is pending
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResetPasswordResultPending {
    /// Point in time (Unix timestamp) after which the password can be reset immediately using resetPassword
    pub pending_reset_date: i32,
}
