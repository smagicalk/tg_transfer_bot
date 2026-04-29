#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of sessions
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Sessions {
    /// List of sessions
    pub sessions: Vec<crate::types::Session>,
    /// Number of days of inactivity before sessions will automatically be terminated; 1-366 days
    pub inactive_session_ttl_days: i32,
}
