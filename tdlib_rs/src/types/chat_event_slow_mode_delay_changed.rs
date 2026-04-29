#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The slow_mode_delay setting of a supergroup was changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventSlowModeDelayChanged {
    /// Previous value of slow_mode_delay, in seconds
    pub old_slow_mode_delay: i32,
    /// New value of slow_mode_delay, in seconds
    pub new_slow_mode_delay: i32,
}
