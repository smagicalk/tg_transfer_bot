#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Returns information about the availability of a temporary password, which can be used for payments
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TemporaryPasswordState {
    /// True, if a temporary password is available
    pub has_password: bool,
    /// Time left before the temporary password expires, in seconds
    pub valid_for: i32,
}
