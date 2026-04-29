#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Story stealth mode settings have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryStealthMode {
    /// Point in time (Unix timestamp) until stealth mode is active; 0 if it is disabled
    pub active_until_date: i32,
    /// Point in time (Unix timestamp) when stealth mode can be enabled again; 0 if there is no active cooldown
    pub cooldown_until_date: i32,
}
