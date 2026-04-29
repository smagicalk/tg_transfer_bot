#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The password reset request was declined
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResetPasswordResultDeclined {
    /// Point in time (Unix timestamp) when the password reset can be retried
    pub retry_date: i32,
}
