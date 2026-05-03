#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A supergroup or channel (with unlimited members)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatTypeSupergroup {
    /// Supergroup or channel identifier
    pub supergroup_id: i64,
    /// True, if the supergroup is a channel
    pub is_channel: bool,
}
