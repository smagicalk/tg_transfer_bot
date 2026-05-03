#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about the period of inactivity after which the current user's account will automatically be deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AccountTtl {
    /// Number of days of inactivity before the account will be flagged for deletion; 30-730 days
    pub days: i32,
}
