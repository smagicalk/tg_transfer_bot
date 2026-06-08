#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about the current recovery email address
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecoveryEmailAddress {
    /// Recovery email address
    pub recovery_email_address: String,
}
